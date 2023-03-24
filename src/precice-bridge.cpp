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

bool SolverInterface::requires_initial_data()
{
  return interface->requiresInitialData();
}

bool SolverInterface::requires_writing_checkpoint()
{
  return interface->requiresWritingCheckpoint();
}

bool SolverInterface::requires_reading_checkpoint()
{
  return interface->requiresReadingCheckpoint();
}

// Mesh Access

bool SolverInterface::has_mesh(::rust::Str mesh_name) const
{
  return interface->hasMesh(to_string(mesh_name));
}
bool SolverInterface::requires_mesh_connectivity_for(::rust::Str mesh_name) const
{
  return interface->requiresMeshConnectivityFor(to_string(mesh_name));
}
vid SolverInterface::set_mesh_vertex(::rust::Str mesh_name, ::rust::Slice<const double> position)
{
  return interface->setMeshVertex(to_string(mesh_name), position.data());
}
vid SolverInterface::get_mesh_vertex_size(::rust::Str mesh_name) const
{
  return interface->getMeshVertexSize(to_string(mesh_name));
}
void SolverInterface::set_mesh_vertices(::rust::Str mesh_name, ::rust::Slice<const double> positions, ::rust::Slice<vid> ids)
{
  interface->setMeshVertices(to_string(mesh_name), positions.size() / interface->getDimensions(), positions.data(), ids.data());
}
void SolverInterface::set_mesh_edge(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id)
{
  return interface->setMeshEdge(to_string(mesh_name), first_vertex_id, second_vertex_id);
}
void SolverInterface::set_mesh_triangle(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id)
{
  interface->setMeshTriangle(to_string(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id);
}
void SolverInterface::set_mesh_quad(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  interface->setMeshQuad(to_string(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}
void SolverInterface::set_mesh_tetrahedron(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  interface->setMeshTetrahedron(to_string(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}

// Data Access

bool SolverInterface::has_data(::rust::Str dataName, ::rust::Str mesh_name) const
{
  return interface->hasData(to_string(dataName), to_string(mesh_name));
}
void SolverInterface::write_block_vector_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> values)
{
  interface->writeBlockVectorData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::write_vector_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> value)
{
  interface->writeVectorData(to_string(mesh_name), to_string(data_name), value_index, value.data());
}
void SolverInterface::write_block_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> values)
{
  interface->writeBlockScalarData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::write_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double value)
{
  interface->writeScalarData(to_string(mesh_name), to_string(data_name), value_index, value);
}
void SolverInterface::read_block_vector_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<double> values) const
{
  interface->readBlockVectorData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::read_vector_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<double> value) const
{
  interface->readVectorData(to_string(mesh_name), to_string(data_name), value_index, value.data());
}
void SolverInterface::read_block_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<double> values) const
{
  interface->readBlockScalarData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), values.data());
}
void SolverInterface::read_scalar_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double &value) const
{
  interface->readScalarData(to_string(mesh_name), to_string(data_name), value_index, value);
}

// experimental: Direct Access

void SolverInterface::set_mesh_access_region(::rust::Str mesh_name, ::rust::Slice<const double> boundingBox)
{
  interface->setMeshAccessRegion(to_string(mesh_name), boundingBox.data());
}
void SolverInterface::get_mesh_vertices_and_ids(::rust::Str mesh_name, ::rust::Slice<vid> ids, ::rust::Slice<double> coordinates) const
{
  interface->getMeshVerticesAndIDs(to_string(mesh_name), ids.size(), ids.data(), coordinates.data());
}

// experimental: Time Interpolation

void SolverInterface::read_block_vector_data_rt(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const
{
  interface->readBlockVectorData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), relativeReadTime, values.data());
}
void SolverInterface::read_vector_data_rt(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double relativeReadTime, ::rust::Slice<double> value) const
{
  interface->readVectorData(to_string(mesh_name), to_string(data_name), value_index, relativeReadTime, value.data());
}
void SolverInterface::read_block_scalar_data_rt(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, double relativeReadTime, ::rust::Slice<double> values) const
{
  interface->readBlockScalarData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), relativeReadTime, values.data());
}
void SolverInterface::read_scalar_data_rt(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, double relativeReadTime, double &value) const
{
  interface->readScalarData(to_string(mesh_name), to_string(data_name), value_index, relativeReadTime, value);
}

// experimental: Gradient Data

bool SolverInterface::requires_gradient_data_for(::rust::Str mesh_name, ::rust::Str data_name) const
{
  return interface->requiresGradientDataFor(to_string(mesh_name), to_string(data_name));
}
void SolverInterface::write_block_vector_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> gradient_values)
{
  interface->writeBlockVectorGradientData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), gradient_values.data());
}
void SolverInterface::write_scalar_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> gradient_values)
{
  interface->writeScalarGradientData(to_string(mesh_name), to_string(data_name), value_index, gradient_values.data());
}
void SolverInterface::write_vector_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, vid value_index, ::rust::Slice<const double> gradient_values)
{
  interface->writeVectorGradientData(to_string(mesh_name), to_string(data_name), value_index, gradient_values.data());
}
void SolverInterface::write_block_scalar_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> valueIndices, ::rust::Slice<const double> gradient_values)
{
  interface->writeBlockScalarGradientData(to_string(mesh_name), to_string(data_name), valueIndices.size(), valueIndices.data(), gradient_values.data());
}

std::unique_ptr<::precice::rust::SolverInterface> create_solverinterface(::rust::Str participant, ::rust::Str config, rint rank, rint size)
{
  return std::make_unique<::precice::rust::SolverInterface>(participant, config, rank, size);
}

} // namespace precice::rust
