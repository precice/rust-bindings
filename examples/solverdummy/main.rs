use precice;
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

    let mut participant = precice::Participant::new(participant_name, config_file_name, 0, 1);

    assert!(participant_name == "SolverOne" || participant_name == "SolverTwo");

    let (mesh_name, read_data_name, write_data_name) = if participant_name == "SolverOne" {
        ("SolverOne-Mesh", "Data-Two", "Data-One")
    } else {
        ("SolverTwo-Mesh", "Data-One", "Data-Two")
    };

    const NUMBER_OF_VERTICES: usize = 3;

    let dimensions = participant.get_mesh_dimensions(mesh_name) as usize;
    let read_dimensions = participant.get_data_dimensions(mesh_name, read_data_name) as usize;
    let write_dimensions = participant.get_data_dimensions(mesh_name, write_data_name) as usize;

    let mut vertices = vec![0_f64; NUMBER_OF_VERTICES * dimensions];
    let mut read_data = vec![0_f64; NUMBER_OF_VERTICES * read_dimensions];
    let mut write_data = vec![0_f64; NUMBER_OF_VERTICES * write_dimensions];

    for i in 0..NUMBER_OF_VERTICES {
        let f = i as f64;
        for j in 0..(dimensions as usize) {
            let idx = j + dimensions * i;
            vertices[idx] = f;
            read_data[idx] = f;
            write_data[idx] = f;
        }
    }

    let vertex_ids: Vec<precice::VertexID> = {
        let mut i32s = vec![0_i32; NUMBER_OF_VERTICES];
        participant.set_mesh_vertices(mesh_name, &vertices, &mut i32s);
        i32s
    };

    if participant.requires_initial_data() {
        println!("DUMMY: Writing initial data\n");
    }

    participant.initialize();

    while participant.is_coupling_ongoing() {
        if participant.requires_writing_checkpoint() {
            println!("DUMMY: Writing iteration checkpoint \n");
        }

        let dt = participant.get_max_time_step_size();
        participant.read_data(mesh_name, read_data_name, &vertex_ids, dt, &mut read_data);

        write_data = read_data.iter().map(|x| x + 1_f64).collect();

        participant.write_data(mesh_name, write_data_name, &vertex_ids, &write_data);

        participant.advance(dt);

        if participant.requires_reading_checkpoint() {
            println!("DUMMY: Reading iteration checkpoint \n");
        } else {
            println!("DUMMY: Advancing in time \n");
        }
    }

    participant.finalize();
    println!("DUMMY: Closing rust solver dummy... \n");

    return ExitCode::SUCCESS;
}
