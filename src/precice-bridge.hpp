#pragma once

#include<memory>
#include<rust/cxx.h>

namespace precice {
  class SolverInterface;
}

namespace precice::rust {

        using rint = int32_t;

class SolverInterface {
  public:

        SolverInterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);
        ~SolverInterface();

        rint get_dimensions() const;
        void finalize();
        void mark_action_fulfilled(::rust::Str action);
        bool is_action_required(::rust::Str action) const;
        double advance(double dt);
        double initialize();
        void initialize_data();
        bool is_coupling_ongoing() const;

        void write_block_scalar_data(rint data_id, ::rust::Slice<const int32_t> vertexIDs, ::rust::Slice<const double> data);
        void read_block_scalar_data(rint data_id, ::rust::Slice<const int32_t> vertexIDs, ::rust::Slice<double> data) const;

        void set_mesh_vertices(rint mesh_id, ::rust::Slice<const double> coordinates, ::rust::Slice<rint> vertexIDs);

        bool is_write_data_required(double dt) const;
        bool is_read_data_available() const;

        rint get_mesh_id(::rust::Str name) const;
        rint get_data_id(::rust::Str name, rint mesh_id) const;

  private:
        std::unique_ptr<::precice::SolverInterface> interface;
};

        ::rust::String action_write_initial_data();
        ::rust::String action_write_iteration_checkpoint();
        ::rust::String action_read_iteration_checkpoint();

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size);

}
