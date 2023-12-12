#include "precice-bridge.hpp"

#include <cstdlib>
#include <memory>
#include <precice/Participant.hpp>
#include <string_view>

namespace {
std::string_view to_string_view(::rust::Str str)
{
  return {str.data(), str.size()};
}
} // namespace

namespace precice::rust {

// Construction

Participant::~Participant() = default;

Participant::Participant(::rust::Str participant, ::rust::Str config, rint rank, rint size)
    : participant(std::make_unique<::precice::Participant>(to_string_view(participant), to_string_view(config), rank, size))
{
}

// Steering Methods
void Participant::initialize()
{
  participant->initialize();
}
void Participant::advance(double dt)
{
  participant->advance(dt);
}
void Participant::finalize()
{
  participant->finalize();
}

// Status Queries
int Participant::get_mesh_dimensions(::rust::Str mesh_name) const
{
  return participant->getMeshDimensions(mesh_name);
}
int Participant::get_data_dimensions(::rust::Str mesh_name, ::rust::Str data_name) const
{
  return participant->getDataDimensions(mesh_name, data_name);
}
bool Participant::is_coupling_ongoing() const
{
  return participant->isCouplingOngoing();
}
bool Participant::is_time_window_complete() const
{
  return participant->isTimeWindowComplete();
}
double Participant::get_max_time_step_size() const
{
  return participant->getMaxTimeStepSize();
}

// Implicit coupling

bool Participant::requires_writing_checkpoint()
{
  return participant->requiresWritingCheckpoint();
}

bool Participant::requires_reading_checkpoint()
{
  return participant->requiresReadingCheckpoint();
}

// Mesh Access

bool Participant::requires_mesh_connectivity_for(::rust::Str mesh_name) const
{
  return participant->requiresMeshConnectivityFor(to_string_view(mesh_name));
}
vid Participant::set_mesh_vertex(::rust::Str mesh_name, ::rust::Slice<const double> position)
{
  return participant->setMeshVertex(to_string_view(mesh_name), position);
}
vid Participant::get_mesh_vertex_size(::rust::Str mesh_name) const
{
  return participant->getMeshVertexSize(to_string_view(mesh_name));
}
void Participant::set_mesh_vertices(::rust::Str mesh_name, ::rust::Slice<const double> positions, ::rust::Slice<vid> ids)
{
  participant->setMeshVertices(to_string_view(mesh_name), positions, ids);
}
void Participant::set_mesh_edge(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id)
{
  return participant->setMeshEdge(to_string_view(mesh_name), first_vertex_id, second_vertex_id);
}
void Participant::set_mesh_edges(::rust::Str mesh_name, ::rust::Slice<const vid> vertices)
{
  participant->setMeshEdges(mesh_name, vertices);
}
void Participant::set_mesh_triangle(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id)
{
  participant->setMeshTriangle(to_string_view(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id);
}
void Participant::set_mesh_triangles(::rust::Str mesh_name, ::rust::Slice<const vid> vertices)
{
  participant->setMeshEdges(mesh_name, vertices);
}
void Participant::set_mesh_quad(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  participant->setMeshQuad(to_string_view(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}
void Participant::set_mesh_quads(::rust::Str mesh_name, ::rust::Slice<const vid> vertices)
{
  participant->setMeshEdges(mesh_name, vertices);
}
void Participant::set_mesh_tetrahedron(::rust::Str mesh_name, int first_vertex_id, int second_vertex_id, int third_vertex_id, int fourth_vertex_id)
{
  participant->setMeshTetrahedron(to_string_view(mesh_name), first_vertex_id, second_vertex_id, third_vertex_id, fourth_vertex_id);
}
void Participant::set_mesh_tetrahedra(::rust::Str mesh_name, ::rust::Slice<const vid> vertices)
{
  participant->setMeshEdges(mesh_name, vertices);
}

// Data Access

bool Participant::requires_initial_data()
{
  return participant->requiresInitialData();
}

void Participant::write_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, ::rust::Slice<const double> values)
{
  participant->writeData(mesh_name, data_name, vertices, values);
}

void Participant::read_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, double relativeReadTime, ::rust::Slice<double> values) const
{
  participant->readData(mesh_name, data_name, vertices, relativeReadTime, values);
}

// experimental: Direct Access

void Participant::set_mesh_access_region(::rust::Str mesh_name, ::rust::Slice<const double> boundingBox)
{
  participant->setMeshAccessRegion(to_string_view(mesh_name), boundingBox);
}
void Participant::get_mesh_vertex_ids_and_coordinates(::rust::Str mesh_name, ::rust::Slice<vid> ids, ::rust::Slice<double> coordinates) const
{
  participant->getMeshVertexIDsAndCoordinates(mesh_name, ids, coordinates);
}

// experimental: Gradient Data

bool Participant::requires_gradient_data_for(::rust::Str mesh_name, ::rust::Str data_name) const
{
  return participant->requiresGradientDataFor(to_string_view(mesh_name), to_string_view(data_name));
}
void Participant::write_gradient_data(::rust::Str mesh_name, ::rust::Str data_name, ::rust::Slice<const vid> vertices, ::rust::Slice<const double> gradients)
{
  participant->writeGradientData(mesh_name, data_name, vertices, gradients);
}

std::unique_ptr<::precice::rust::Participant> create_participant(::rust::Str participant, ::rust::Str config, rint rank, rint size)
{
  return std::make_unique<::precice::rust::Participant>(participant, config, rank, size);
}

} // namespace precice::rust
