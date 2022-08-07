use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use anyhow::Result;

use crate::{TransactionRef, Transactions};

#[derive(Debug, Default, Clone)]
pub struct TxSortedList {
    txs: BTreeMap<u64, TransactionRef>,
}

impl TxSortedList {
    pub fn new() -> Self {
        Self {
            txs: Default::default(),
        }
    }
    pub fn put(&mut self, tx: TransactionRef) {
        self.txs.insert(tx.nonce(), tx);
    }

    pub fn get(&self, nonce: u64) -> Option<&TransactionRef> {
        self.txs.get(&nonce)
    }

    pub fn remove(&mut self, nonce: u64) -> bool {
        self.txs.remove(&nonce).is_some()
    }

    pub fn filter<F>(&mut self, f: F) -> Transactions
    where
        F: FnMut(&u64, &mut TransactionRef) -> bool,
    {
        self.txs.drain_filter(f).map(|(_, tx)| tx).collect()
    }
    pub fn forward(&mut self, threshold: u64) -> Vec<TransactionRef> {
        self.txs
            .drain_filter(|nonce, _| *nonce < threshold)
            .map(|(_, tx)| tx)
            .collect()
    }

    pub fn ready(&mut self, start: u64) -> Vec<TransactionRef> {
        self.txs
            .drain_filter(|nonce, _| *nonce >= start)
            .map(|(_, tx)| tx)
            .collect()
    }

    pub fn cap(&mut self, threshold: usize) -> Vec<TransactionRef> {
        if self.txs.len() <= threshold {
            println!("self.txs.len() <= threshold returning");
            return Default::default();
        }
        println!("Cap <= {} returning", threshold);
        let mut remain = BTreeMap::new();
        let mut slots = threshold;
        while slots > 0 {
            if let Some((tx_hash, tx)) = self.txs.pop_first() {
                remain.insert(tx_hash, tx);
                slots -= 1;
            } else {
                break;
            }
        }
        std::mem::swap(&mut remain, &mut self.txs);
        let drops: Vec<_> = remain
            .iter()
            .map(|(_, priced_tx)| priced_tx.clone())
            .collect();
        drops
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.txs.is_empty()
    }

    pub fn last_element(&self) -> Option<TransactionRef> {
        self.txs.first_key_value().map(|(_, tx)| tx.clone())
    }

    pub fn flatten(&self) -> Vec<TransactionRef> {
        self.txs.iter().map(|(_, tx)| tx.clone()).collect()
    }
    pub fn overlaps(&self, nonce: u64) -> bool {
        self.txs.contains_key(&nonce)
    }
    pub fn has(&self, nonce: u64) -> bool {
        self.txs.contains_key(&nonce)
    }
}

#[derive(Debug)]
pub struct TxList {
    strict: bool,
    pub(crate) txs: TxSortedList,
}

impl TxList {
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            txs: Default::default(),
        }
    }
    pub fn add(&mut self, tx: TransactionRef, price_bump: u128) -> (bool, Option<TransactionRef>) {
        let old = self.txs.get(tx.nonce()).cloned();
        if let Some(old) = &old {
            let old_fees = old.fees();
            let bump = ((tx.fees() as i128 - old_fees as i128) * 100) / tx.fees() as i128;
            if old.fees().cmp(&tx.fees()).is_le() && bump < price_bump as i128 {
                return (false, None);
            }
        }
        self.txs.put(tx);
        (true, old)
    }

    pub fn remove(&mut self, tx: TransactionRef) -> (bool, Transactions) {
        let nonce = tx.nonce();
        if self.txs.remove(nonce) {
            return (false, Vec::new());
        }
        if self.strict {
            return (true, self.txs.filter(|_, tx| tx.nonce() > nonce));
        }

        (true, Vec::new())
    }

    pub fn forward(&mut self, threshold: u64) -> Vec<TransactionRef> {
        self.txs.forward(threshold)
    }

    pub fn filter(
        &mut self,
        price_limit: u128,
    ) -> Option<(Vec<TransactionRef>, Vec<TransactionRef>)> {
        if price_limit == 0 {
            return None;
        }

        let removed = self.txs.filter(|_, tx| tx.price() > price_limit);

        let mut invalids = Vec::new();

        if self.strict {
            let mut lowest = u64::MAX;
            for tx in removed.iter() {
                let nonce = tx.nonce();
                if lowest > nonce {
                    lowest = nonce;
                }
            }
            invalids = self.txs.filter(|_, tx| tx.nonce() > lowest);
        }

        Some((removed, invalids))
    }

    pub fn ready(&mut self, start: u64) -> Vec<TransactionRef> {
        self.txs.ready(start)
    }

    pub fn cap(&mut self, threshold: u64) -> Vec<TransactionRef> {
        self.txs.cap(threshold as usize)
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.txs.is_empty()
    }

    pub fn flatten(&self) -> Vec<TransactionRef> {
        self.txs.flatten()
    }

    pub fn last_element(&self) -> Option<TransactionRef> {
        self.txs.last_element()
    }
    pub fn overlaps(&self, tx: TransactionRef) -> bool {
        self.txs.overlaps(tx.nonce())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PricedTransaction(TransactionRef);

impl Eq for PricedTransaction {}

impl PartialEq for PricedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for PricedTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.fees().partial_cmp(&other.0.fees()) {
            None => None,
            Some(comp) => match comp.reverse() {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Equal => self
                    .0
                    .nonce()
                    .partial_cmp(&other.0.nonce())
                    .map(|ord| ord.reverse()),
                Ordering::Greater => Some(Ordering::Greater),
            },
        }
    }
}

impl Ord for PricedTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.fees().cmp(&other.0.fees()).reverse() {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.0.nonce().cmp(&other.0.nonce()).reverse(),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NonceTransaction(pub(crate) TransactionRef);

impl Eq for NonceTransaction {}

impl PartialEq for NonceTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for NonceTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.nonce().cmp(&other.0.nonce()).reverse())
    }
}

impl Ord for NonceTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.nonce().cmp(&other.0.nonce()).reverse()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TxPricedList {
    txs: BTreeSet<PricedTransaction>,
    total_fee: Arc<AtomicUsize>,
}

impl TxPricedList {
    pub fn new() -> Self {
        Self {
            txs: Default::default(),
            total_fee: Default::default(),
        }
    }

    pub fn put(&mut self, tx: TransactionRef, is_local: bool) {
        if is_local {
            return;
        }
        self.total_fee
            .fetch_add(tx.fees() as usize, std::sync::atomic::Ordering::Relaxed);
        let _ = self.txs.insert(PricedTransaction(tx));
    }

    pub fn remove(&mut self, tx: TransactionRef) -> bool {
        let tx_fee = tx.fees() as usize;
        let removed = self.txs.remove(&PricedTransaction(tx));
        if removed {
            self.total_fee
                .fetch_sub(tx_fee, std::sync::atomic::Ordering::Relaxed);
        }
        removed
    }

    pub fn underpriced(&self, tx: TransactionRef) -> Result<bool> {
        let least_priced_tx = match self.txs.last() {
            None => {
                return Ok(false);
            }
            Some(tx) => tx,
        };
        Ok(least_priced_tx.cmp(&PricedTransaction(tx)).is_ge())
    }

    pub fn discard(&mut self, slots: u64) -> Result<Vec<TransactionRef>> {
        if self.txs.len() <= slots as usize {
            return Ok(Default::default());
        }
        let mut remain = BTreeSet::new();
        let mut slots = slots;
        while slots > 0 {
            if let Some(tx) = self.txs.pop_first() {
                remain.insert(tx);
                slots -= 1;
            } else {
                break;
            }
        }
        std::mem::swap(&mut remain, &mut self.txs);
        let drops: Vec<_> = remain.iter().map(|priced_tx| priced_tx.0.clone()).collect();
        Ok(drops)
    }
}

#[cfg(test)]
mod tests {

    use account::create_account;

    use crate::tests::make_tx;
    use crate::tx_list::TxList;

    #[test]
    fn test_txlist() {
        let alice = create_account();
        let bob = create_account();
        let mut list = TxList::new(true);
        list.add(make_tx(&alice, &bob, 1, 100, 0), 10);
        list.add(make_tx(&alice, &bob, 2, 200, 0), 10);
        list.add(make_tx(&alice, &bob, 3, 300, 0), 10);
        list.add(make_tx(&alice, &bob, 5, 400, 0), 10);
        list.add(make_tx(&alice, &bob, 6, 500, 0), 10);
        list.add(make_tx(&alice, &bob, 7, 600, 0), 10);
        list.add(make_tx(&alice, &bob, 8, 800, 0), 10);
        list.add(make_tx(&alice, &bob, 9, 900, 0), 10);

        //let removed = list.forward(3);
        let cap = list.cap(4);
        println!("cap {:#?}", cap);
        println!("remaining {:#?}", list.txs);
        // assert_eq!(removed.len(), 2);
        // assert_eq!(removed.range(3..).count(), 0);
        // assert_eq!(list.txs.write().unwrap().range(..3).count(), 0);

        // let mut priced_list = TxPricedList::new();
        // priced_list.put(make_tx(&alice, &bob, 1, 40, 4), false);
        // priced_list.put(make_tx(&alice, &bob, 2, 20, 2), false);
        // priced_list.put(make_tx(&alice, &bob, 3, 30, 3), false);
        // priced_list.put(make_tx(&alice, &bob, 4, 40, 4), false);
        // priced_list.put(make_tx(&bob, &alice, 8, 100, 10), false);
        // priced_list.put(make_tx(&bob, &alice, 9, 100, 10), false);

        // println!("{:#?}", priced_list);
        // println!("-------------------------------------------------------------------------------------------------------");
        // println!("{:#?}", priced_list.discard(4).unwrap());
    }
}
