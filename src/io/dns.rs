use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::{GenericConnector, TokioConnectionProvider};
use hickory_resolver::proto::runtime::TokioRuntimeProvider;
use hickory_resolver::Resolver;
use once_cell::sync::Lazy;

pub static RESOLVER: Lazy<Resolver<GenericConnector<TokioRuntimeProvider>>> = Lazy::new(|| {
    let resolver = Resolver::builder_with_config(
        ResolverConfig::cloudflare(),
        TokioConnectionProvider::default()
    ).build();
    
    resolver
});