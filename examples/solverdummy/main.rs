use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("The solverdummy was called with an incorrect number of arguments. Usage: ./solverdummy configFile solverName\n\n");
        println!("Parameter description\n");
        println!("  configurationFile: Path and filename of preCICE configuration\n");
        println!("  solverName:        SolverDummy participant name in preCICE configuration\n");
        return ExitCode::from(1);
    }

    let config_file_name = &args[1];
    let participant_name = &args[2];

    println!("DUMMY: Running solver dummy with preCICE config file \"{}\" and participant name \"{}\".\n", config_file_name, participant_name);

    let mut interface = precice::new(participant_name, config_file_name, 0, 1);

    assert!(participant_name == "SolverOne" || participant_name == "SolverTwo");

    let (mesh_name, read_data_name, write_data_name) = if participant_name == "SolverOne" {
        ("MeshOne", "dataTwo", "dataOne")
    } else {
        ("MeshTwo", "dataOne", "dataTwo")
    };

    let mesh_id = interface.get_mesh_id(mesh_name);
    let write_data_id = interface.get_data_id(write_data_name, mesh_id);
    let read_data_id = interface.get_data_id(read_data_name, mesh_id);
    const NUMBER_OF_VERTICES: usize = 3;

    let dimensions = interface.get_dimensions() as usize;
    let mut vertices = vec![0_f64; NUMBER_OF_VERTICES * dimensions];
    let mut read_data = vec![0_f64; NUMBER_OF_VERTICES * dimensions];
    let mut write_data = vec![0_f64; NUMBER_OF_VERTICES * dimensions];

    for i in 0..NUMBER_OF_VERTICES {
        let f = i as f64;
        for j in 0..(dimensions as usize) {
            let idx = j + dimensions * i;
            vertices[idx] = f;
            read_data[idx] = f;
            write_data[idx] = f;
        }
    }

    let vertex_ids = {
        let mut vids = vec![0_i32; NUMBER_OF_VERTICES];
        interface
            .pin_mut()
            .set_mesh_vertices(mesh_id, &vertices, &mut vids);
        vids
    };

    let mut dt = interface.pin_mut().initialize();

    if interface.is_action_required(&precice::action_write_initial_data()) {
        println!("DUMMY: Writing initial data\n");
        interface
            .pin_mut()
            .mark_action_fulfilled(&precice::action_write_initial_data());
    }

    interface.pin_mut().initialize_data();

    while interface.is_coupling_ongoing() {
        if interface.is_action_required(&precice::action_write_iteration_checkpoint()) {
            println!("DUMMY: Writing iteration checkpoint \n");
            interface
                .pin_mut()
                .mark_action_fulfilled(&precice::action_write_iteration_checkpoint());
        }

        interface.read_block_vector_data(read_data_id, &vertex_ids, &mut read_data);

        write_data = read_data.iter().map(|x| x + 1_f64).collect();

        interface
            .pin_mut()
            .write_block_vector_data(write_data_id, &vertex_ids, &write_data);

        dt = interface.pin_mut().advance(dt);

        if interface.is_action_required(&precice::action_read_iteration_checkpoint()) {
            println!("DUMMY: Reading iteration checkpoint \n");
            interface
                .pin_mut()
                .mark_action_fulfilled(&precice::action_read_iteration_checkpoint());
        } else {
            println!("DUMMY: Advancing in time \n");
        }
    }

    interface.pin_mut().finalize();
    println!("DUMMY: Closing rust solver dummy... \n");

    return ExitCode::SUCCESS;
}
