use std::collections::BTreeMap;
use std::sync::Arc;
use odana_rt_api::Executable;

pub struct AppsRepository {
    apps: BTreeMap<u64, Arc<Executable>>,
    // TODO: install app by block height
}

impl AppsRepository {
    fn get_app(&self, id: u64) -> Option<&Arc<Executable>> {
        self.apps.get(&id)
    }

    fn install_app(&mut self, id: u64, exec: Executable) {
        self.apps.insert(id, Arc::new(exec));
    }
}
