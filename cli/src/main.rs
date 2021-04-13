mod diff;
mod file;
mod patch;
mod plan;

use patch::Patch;
use plan::Plan;

fn main() {
    let source = String::from("/home/dezyh/coding/patchup/cli/test/old");
    let target = String::from("/home/dezyh/coding/patchup/cli/test/new");
    let output = String::from("/home/dezyh/coding/patchup/cli/test/patch.up");

    let plan = Plan::new(source.clone(), target.clone());
    plan.print();

    let patch = Patch::from_plan(plan);
    patch.write_file(&output);

    let patch = Patch::from_file(output);
    patch.apply(&source);
}
