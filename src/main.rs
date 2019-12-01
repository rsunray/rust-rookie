mod topics;

use topics::variables::demo_variables;
use topics::data_types::demo_data_types;
use topics::functions::demo_functions;
use topics::control_flow::demo_control_flow;

fn main() {
    demo_variables();

    demo_data_types();
    
    demo_functions();

    demo_control_flow();
}

