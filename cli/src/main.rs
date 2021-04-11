mod plan;
mod file;

use plan::Plan;

fn main() {
    let source = String::from("/home/dezyh/coding/patchup/cli/test/old");
    let target = String::from("/home/dezyh/coding/patchup/cli/test/new");
    
    let plan = Plan::new(source, target);

    plan.print();

}
