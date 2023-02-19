#include "precice-bridge.hpp"

#include <cstdlib>
#include <memory>
#include <precice/SolverInterface.hpp>
#include <string>

namespace {
std::string to_string(::rust::Str str)
{
  return static_cast<std::string>(str);
}
} // namespace

namespace precice::rust {

// Construction

SolverInterface::~SolverInterface() = default;

SolverInterface::SolverInterface(::rust::Str participant, ::rust::Str config, rint rank, rint size)
    : interface(std::make_unique<::precice::SolverInterface>(to_string(participant), to_string(config), rank, size))
{
}

// Steering Methods
double SolverInterface::advance(double dt)
{
  return interface->advance(dt);
}
double SolverInterface::initialize()
{
  return interface->initialize();
}
void SolverInterface::initialize_data()
{
  interface->initializeData();
}
void SolverInterface::finalize()
{
  interface->finalize();
}

// Status Queries
rint SolverInterface::get_dimensions() const
{
  return interface->getDimensions();
}
bool SolverInterface::is_coupling_ongoing() const
{
  return interface->isCouplingOngoing();
}
bool SolverInterface::is_time_window_complete() const
{
  return interface->isTimeWindowComplete();
}

// Action Methods

void SolverInterface::mark_action_fulfilled(::rust::Str action)
{
  interface->markActionFulfilled(to_string(action));
}
bool SolverInterface::is_action_required(::rust::Str action) const
{
  return interface->isActionRequired(to_string(action));
}

// Mesh Access

bool SolverInterface::has_mesh(::rust::Str mesh_name) const
{
  return interface->hasMesh(to_string(mesh_name));
}
rint SolverInterface::get_mesh_id(::rust::Str name) const
{
  return interface->getMeshID(to_string(name));
}
bool SolverInterface::is_mesh_connectivity_required(rint mesh_id) const
{
  return interface->isMeshConnectivityRequired(mesh_id);
}
rint SolverInterface::set_mesh_vertex(rint mesh_id, ::rust::Slice<const double> position)
{
  return interface->setMeshVertex(mesh_id, position.data());
}
rint SolverInterface::get_mesh_vertex_size(rint mesh_id) const
{
  return interface->getMeshVertexSize(mesh_id);
}
void SolverInterface::set_mesh_vertices(rint mesh_id, ::rust::Slice<const double> positions, ::rust::Slice<rint> ids)
{
  interface->setMeshVertices(mesh_id, positions.size() / interface->getDimensions(), positions.data(), ids.data());
}
rint SolverInterface::set_mesh_edge(rint mesh_id, int first_vertex_id, int second_vertex_id)
{
  return interface->setMeshEdge(mesh_id, first_vertex_id, second_vertex_id);
}
void SolverInterface::set_mesh_triangle(rint mesh_id, int first_edge_id, int second_edge_id, int third_edge_id)
{
  interface->setMeshTriangle(mesh_id, first_edge_id, second_edge_id, third_edge_id);
}
void SolverInterface::set_mesh_triangle_with_edges(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id)
{
  interface->setMeshTriangleWithEdges(mesh_id, first_vertex_id, second_vertex_id, third_vertex_id);
}
void SolverInterface::set_mesh_quad(rint mesh_id, int first_edge_id, int second_edge_id, int third_edge_id, int fourth_edge_id)
{
  interface->setMeshQuad(mesh_id, first_edge_id, second_edge_id, third_edge_id, fourth_edge_id);
}
void SolverInterface::set_mesh_quad_with_edges(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  interface->setMeshQuadWithEdges(mesh_id, first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}
void SolverInterface::set_mesh_tetrahedron(rint mesh_id, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  interface->setMeshTetrahedron(mesh_id, first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}

// Data Access

bool SolverInterface::has_data(::rust::Str dataName, rint mesh_id) const
{
  return interface->hasData(to_string(dataName), mesh_id);
}
rint SolverInterface::get_data_id(::rust::Str name, rint mesh_id) const
{
  return interface->getDataID(to_string(name), mesh_id);
}
void SolverInterface::write_block_vector_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> values)
{
  interface->writeBlockVectorData(data_id, valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::write_vector_data(rint data_id, rint value_index, ::rust::Slice<const double> value)
{
  interface->writeVectorData(data_id, value_index, value.data());
}
void SolverInterface::write_block_scalar_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> values)
{
  interface->writeBlockScalarData(data_id, valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::write_scalar_data(rint data_id, rint value_index, double value)
{
  interface->writeScalarData(data_id, value_index, value);
}
void SolverInterface::read_block_vector_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<double> values) const
{
  interface->readBlockVectorData(data_id, valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::read_vector_data(rint data_id, rint value_index, ::rust::Slice<double> value) const
{
  interface->readVectorData(data_id, value_index, value.data());
}
void SolverInterface::read_block_scalar_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<double> values) const
{
  interface->readBlockScalarData(data_id, valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::read_scalar_data(rint data_id, rint value_index, double &value) const
{
  interface->readScalarData(data_id, value_index, value);
}

// experimental: Direct Access

void SolverInterface::set_mesh_access_region(rint mesh_id, ::rust::Slice<const double> boundingBox)
{
  interface->setMeshAccessRegion(mesh_id, boundingBox.data());
}
void SolverInterface::get_mesh_vertices_and_ids(rint mesh_id, ::rust::Slice<rint> ids, ::rust::Slice<double> coordinates) const
{
  interface->getMeshVerticesAndIDs(mesh_id, ids.size(), ids.data(), coordinates.data());
}

// experimental: Time Interpolation

void SolverInterface::read_block_vector_data_rt(rint data_id, ::rust::Slice<const rint> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const
{
  interface->readBlockVectorData(data_id, valueIndices.size(), valueIndices.data(), relativeReadTime, values.data());
}
void SolverInterface::read_vector_data_rt(rint data_id, rint value_index, double relativeReadTime, ::rust::Slice<double> value) const
{
  interface->readVectorData(data_id, value_index, relativeReadTime, value.data());
}
void SolverInterface::read_block_scalar_data_rt(rint data_id, ::rust::Slice<const rint> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const
{
  interface->readBlockScalarData(data_id, valueIndices.size(), valueIndices.data(), relativeReadTime, values.data());
}
void SolverInterface::read_scalar_data_rt(rint data_id, rint value_index, double relativeReadTime, double &value) const
{
  interface->readScalarData(data_id, value_index, relativeReadTime, value);
}

// experimental: Gradient Data

bool SolverInterface::is_gradient_data_required(rint data_id) const
{
  return interface->isGradientDataRequired(data_id);
}
void SolverInterface::write_block_vector_gradient_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> gradient_values)
{
  interface->writeBlockVectorGradientData(data_id, valueIndices.size(), valueIndices.data(), gradient_values.data());
}
void SolverInterface::write_scalar_gradient_data(rint data_id, rint value_index, ::rust::Slice<const double> gradient_values)
{
  interface->writeScalarGradientData(data_id, value_index, gradient_values.data());
}
void SolverInterface::write_vector_gradient_data(rint data_id, rint value_index, ::rust::Slice<const double> gradient_values)
{
  interface->writeVectorGradientData(data_id, value_index, gradient_values.data());
}
void SolverInterface::write_block_scalar_gradient_data(rint data_id, ::rust::Slice<const rint> valueIndices, ::rust::Slice<const double> gradient_values)
{
  interface->writeBlockScalarGradientData(data_id, valueIndices.size(), valueIndices.data(), gradient_values.data());
}

::rust::String action_write_initial_data()
{
  return ::precice::constants::actionWriteInitialData();
}
::rust::String action_write_iteration_checkpoint()
{
  return ::precice::constants::actionWriteIterationCheckpoint();
}
::rust::String action_read_iteration_checkpoint()
{
  return ::precice::constants::actionReadIterationCheckpoint();
}

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size)
{
  return std::make_unique<::precice::rust::SolverInterface>(participant, config, rank, size);
}

} // namespace precice::rust
