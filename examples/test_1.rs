use identity_iota::{
    core::{Object, Url},
    did::{CoreDID, DIDUrl, DID},
    document::{CoreDocument, DocumentBuilder, Service},
    resolver::Resolver,
};

async fn foo_handler(did: CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
    let service: Service = Service::builder(Object::new())
        .id(DIDUrl::parse("did:example:123#service").unwrap())
        .service_endpoint(Url::parse("https://example.com/").unwrap())
        .type_("foo")
        .build()
        .unwrap();
    let core_doc = DocumentBuilder::default()
        .id(did)
        .service(service)
        .build()
        .unwrap();
    Ok(core_doc)
}

async fn bar_handler(did: CoreDID) -> std::result::Result<CoreDocument, std::io::Error> {
    let service: Service = Service::builder(Object::new())
        .id(DIDUrl::parse("did:example:123#service").unwrap())
        .service_endpoint(Url::parse("https://example/").unwrap())
        .type_("bar")
        .build()
        .unwrap();
    let core_doc = DocumentBuilder::default()
        .id(did)
        .service(service)
        .build()
        .unwrap();
    Ok(core_doc)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut resolver_core: Resolver<CoreDocument> = Resolver::new();

    let did_1: CoreDID = CoreDID::parse("did:foo:1234").unwrap();
    let did_2: CoreDID = CoreDID::parse("did:foo:bar:1234").unwrap();
    resolver_core.attach_handler(did_1.method().to_owned(), foo_handler);
    resolver_core.attach_handler(did_2.method().to_owned(), bar_handler);

    let resolved_foo = resolver_core.resolve(&did_1).await?;
    println!("{:?}", resolved_foo); //service typ is "foo".
    let resolved_bar = resolver_core.resolve(&did_2).await?;
    println!("{:?}", resolved_bar); //service typ is "bar"

    Ok(())
}
