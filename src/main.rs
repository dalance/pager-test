use pager::Pager;

fn main() {
    Pager::with_pager("less -SR")
        .pager_envs(&["LESSCHARSET=utf-8"])
        .setup();
    println!("Hello, world!");
}
