extern crate async_oauth2;

#[cfg(test)]
extern crate rspec;
extern crate dotenv;
extern crate envy;


use rspec::given;

#[test]
fn explicit_flow() {
    rspec::run(&given("a Oauth2 Common Flow", 0, |ctx| {
        ctx.it("then runs some specs", |_| {
            true
        });
    }));
}
