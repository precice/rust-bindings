#include "precice-bridge.hpp"

#include <memory>
#include <string>
#include <precice/SolverInterface.hpp>

namespace {
  std::string to_string(::rust::Str str) {
    return static_cast<std::string>(str);
  }
}

namespace precice::rust {

      SolverInterface::~SolverInterface() = default;

  SolverInterface::SolverInterface(::rust::Str participant, ::rust::Str config, rint rank, rint size)
    : interface(std::make_unique<::precice::SolverInterface>(to_string(participant), to_string(config), rank, size))
  {}

        rint SolverInterface::get_dimensions() const { return interface->getDimensions(); }
        void SolverInterface::finalize() { interface->finalize(); }
        void SolverInterface::mark_action_fulfilled(::rust::Str action) { interface->markActionFulfilled(to_string(action)); }
        bool SolverInterface::is_action_required(::rust::Str action) const { return interface->isActionRequired(to_string(action));}
        double SolverInterface::advance(double dt) { return interface->advance(dt); }
        double SolverInterface::initialize() { return interface->initialize(); }
        void SolverInterface::initialize_data() { interface->initializeData(); }
        bool SolverInterface::is_coupling_ongoing() const { return interface->isCouplingOngoing(); }

        void SolverInterface::write_block_scalar_data(rint data_id, ::rust::Slice<const int32_t> vertexIDs, ::rust::Slice<const double> data)
        {
          interface->writeBlockScalarData(data_id, vertexIDs.size(), vertexIDs.data(), data.data());
        }

        void SolverInterface::read_block_scalar_data(rint data_id, ::rust::Slice<const int32_t> vertexIDs, ::rust::Slice<double> data) const {
          interface->readBlockScalarData(data_id, vertexIDs.size(), vertexIDs.data(), data.data());
        }

        void SolverInterface::set_mesh_vertices(rint mesh_id, ::rust::Slice<const double> coordinates, ::rust::Slice<rint> vertexIDs) {
          interface->setMeshVertices(mesh_id, vertexIDs.size(), coordinates.data(), vertexIDs.data());
        }

        bool SolverInterface::is_write_data_required(double dt) const { return interface->isWriteDataRequired(dt); }
        bool SolverInterface::is_read_data_available() const { return interface->isReadDataAvailable(); }

        rint SolverInterface::get_mesh_id(::rust::Str name) const { return interface->getMeshID(to_string(name)); }
        rint SolverInterface::get_data_id(::rust::Str name, rint mesh_id) const { return interface->getDataID(to_string(name), mesh_id); }

        ::rust::String action_write_initial_data() { return ::precice::constants::actionWriteInitialData(); }
        ::rust::String action_write_iteration_checkpoint() { return ::precice::constants::actionWriteIterationCheckpoint(); }
        ::rust::String action_read_iteration_checkpoint() {return ::precice::constants::actionReadIterationCheckpoint(); }

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size)
{
  return std::make_unique<::precice::rust::SolverInterface>(participant,config,rank,size);
}

}
