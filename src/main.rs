extern crate mrusty;

use std::fs::File;

use mrusty::*;

fn main() {
    let mruby = Mruby::new();

    mruby.run(r#"
        class HashObj
            def initialize(hash)
                hash.each do |key, val|
                    self.instance_variable_set(
                        "@#{key}",
                        if val.is_a?(Hash) then
                            HashObj.new(val)
                        else
                            val
                        end
                    )

                    self.class.define_method key do
                        self.instance_variable_get("@#{key}")
                    end
                end
            end
        end
    "#).unwrap();

    let site = {
        use std::io::Read;

        let mut f = File::open("./examples/test-site/site.rb").unwrap();
        let mut buffer = String::new();

        f.read_to_string(&mut buffer).unwrap();
        buffer
    };

    println!(
        "{:?}",
        mruby.run(&site)
            .unwrap()
            .call(
                "call",
                vec![mruby.nil(), mruby.nil()]
            )
    );
}
