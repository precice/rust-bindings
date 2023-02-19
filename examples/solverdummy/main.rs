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

    let configFileName = &args[1];
    let participantName = &args[2];

    println!("DUMMY: Running solver dummy with preCICE config file \"{}\" and participant name \"{}\".\n", configFileName, participantName);

    let mut interface = precice::new(participantName, configFileName, 0, 1);

    assert!(participantName == "SolverOne" || participantName == "SolverTwo");

    let (meshName, readDataName, writeDataName) = if participantName == "SolverOne" {
        ("MeshOne", "dataTwo", "dataOne")
    } else {
        ("MeshTwo", "dataOne", "dataTwo")
    };

    let meshID = interface.get_mesh_id(meshName);
    let writeDataID = interface.get_data_id(writeDataName, meshID);
    let readDataID = interface.get_data_id(readDataName, meshID);
    const numberOfVertices: usize = 3;

    let dimensions = interface.get_dimensions() as usize;
    let mut vertices = vec![0_f64; numberOfVertices * dimensions];
    let mut readData = vec![0_f64; numberOfVertices * dimensions];
    let mut writeData = vec![0_f64; numberOfVertices * dimensions];

    for i in 0..numberOfVertices {
        let f = i as f64;
        for j in 0..(dimensions as usize) {
            let idx = j + dimensions * i;
            vertices[idx] = f;
            readData[idx] = f;
            writeData[idx] = f;
        }
    }

    let vertexIDs = {
        let mut vids = vec![0_i32; numberOfVertices];
        interface
            .pin_mut()
            .set_mesh_vertices(meshID, &vertices, &mut vids);
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

        interface.read_block_vector_data(readDataID, &vertexIDs, &mut readData);

        writeData = readData.iter().map(|x| x + 1_f64).collect();

        interface
            .pin_mut()
            .write_block_vector_data(writeDataID, &vertexIDs, &writeData);

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
