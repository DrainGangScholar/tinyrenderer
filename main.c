#include <stdio.h>
#include <stdlib.h>
struct vec3f {
  float x, y, z;
} typedef vec3f;
struct vec3u {
  unsigned int x, y, z;
} typedef vec3u;
struct vec2f {
  float s, t;
} typedef vec2f;
struct vec3 {
  vec3f *data;
  unsigned int capacity;
  unsigned int count;
} typedef vec;
struct Face {
  vec3u ind;
  unsigned int count;
} typedef Face;
Face *_face(unsigned int x, unsigned int y, unsigned int z) {
  Face *face = (Face *)malloc(sizeof(Face));
  face->ind.x = x;
  face->ind.y = y;
  face->ind.z = z;
  face->count = 3;
  return face;
}
struct Model {
  vec *vertices;
  Face *faces;
  const char *path;
} typedef Model;
struct Model *read(const char *path) {
  Model *model = (Model *)malloc(sizeof(Model));
  FILE *file;
  char line[100];
  file = fopen(path, "r");
  if (file == NULL) {
    printf("file can't be opened or path is bad\n");
    exit(1);
  }
  char first, second;
  while (fgets(line, sizeof(line), file) != NULL) {
    first = line[0], second = line[1];
    // vertex
    if (first == 'v') {
      // regular
      if (second == ' ') {
        float x, y, z;
        sscanf(line, "v %f %f %f", &x, &y, &z);
        vec3f vertex = {x, y, z};
        printf("Vertex x:%f,y:%f,z:%f\n", vertex.x, vertex.y, vertex.z);
      }
      // texture
      else if (second == 't') {
        float s, t;
        sscanf(line, "vt %f %f", &s, &t);
        vec2f texture = {s, t};
        printf("Texture s:%f,t:%f\n", texture.s, texture.t);
      }
      // normal
      else if (second == 'n') {
        float x, y, z;
        sscanf(line, "v %f %f %f", &x, &y, &z);
        vec3f normal = {x, y, z};
        printf("Normal x:%f,y:%f,z:%f\n", normal.x, normal.y, normal.z);
      }
    }
    // face
    else if (first == 'f') {
      unsigned int x, y, z;
      sscanf(line, "f %u/%*u/%*u %u/%*u/%*u %u/%*u/%*u", &x, &y, &z);
      Face *face = _face(x, y, z);
      printf("Face x:%u,y:%u,z:%u\n", face->ind.x, face->ind.y, face->ind.z);
    }
  }
  model->path = path;
  fclose(file);
  return model;
}
int main(int argc, char *argv[]) {
  const char *path = argv[1];
  Model *model = read(path);
  return 0;
}
