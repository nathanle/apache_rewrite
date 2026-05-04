//use mod_rewrite::Engine;
//#[path = "mod_rewrite/src/"]
//mod mod_rewrite; 
//pub use crate::mod_rewrite::Engine;
pub use mod_rewrite::Engine;
/*
        RewriteRule ^/3M/ru_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/3M/ru_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302]
        RewriteRule ^/my3M/ru_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/3M/en_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/my3M/en_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/3M/by_RY(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/my3M/by_RY(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/3M/ru_BY(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/my3M/ru_BY(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^(.*)jndi%3Aldap(.*)$ - [R=404]
        RewriteRule ^(.*)jndi:ldap(.*)$ - [R=404]
        RewriteRule ^(.*)jndi%3Armi(.*)$ - [R=404]
        RewriteRule ^(.*)jndi%3Adns(.*)$ - [R=404]
        RewriteRule ^(.*)jndi:dns(.*)$ - [R=404]
        RewriteRule ^(.*)jndi:rmi(.*)$ - [R=404]
        RewriteRule ^/3M/ru_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^/3M/ru_RU(.*)$ https://www.abc.com/1M/en_US/select-location/? [R=302,NS,NC,NE]
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::default();
    engine.add_rules(r#"
        RewriteRule ^/3M/ru_RU(.*)$ https://www.3m.com/3M/en_US/select-location/? [R=302,L,NS,NC,NE]
        RewriteRule ^(.*)jndi:rmi(.*)$ - [R=404]
    "#)?;

    let list = [
        "/3M/ru_RU/test/west/left.html",
        "https://www.3m.com/jndi:rmi/west/left.html"
    ];
    for uri in list {
        let result = engine.rewrite(uri).unwrap();
        println!("{result:?}");
    }
    Ok(())
}
