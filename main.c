#include <stdio.h>
#include <stdlib.h>
#include <time.h>
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
  unsigned int _vertex = 0, _texture = 0, _normal = 0, __face = 0;
  while (fgets(line, sizeof(line), file) != NULL) {
    first = line[0], second = line[1];
    // vertex
    if (first == 'v') {
      // regular
      if (second == ' ') {
        float x, y, z;
        sscanf(line, "v %f %f %f", &x, &y, &z);
        vec3f vertex = {x, y, z};
        //    printf("Vertex x:%f,y:%f,z:%f\n", vertex.x, vertex.y, vertex.z);
        _vertex++;
      }
      // texture
      else if (second == 't') {
        float s, t;
        sscanf(line, "vt %f %f", &s, &t);
        vec2f texture = {s, t};
        //     printf("Texture s:%f,t:%f\n", texture.s, texture.t);
        _texture++;
      }
      // normal
      else if (second == 'n') {
        float x, y, z;
        sscanf(line, "v %f %f %f", &x, &y, &z);
        vec3f normal = {x, y, z};
        //      printf("Normal x:%f,y:%f,z:%f\n", normal.x, normal.y, normal.z);
        _normal++;
      }
    }
    // face
    else if (first == 'f') {
      unsigned int x, y, z;
      sscanf(line, "f %u/%*u/%*u %u/%*u/%*u %u/%*u/%*u", &x, &y, &z);
      Face *face = _face(x, y, z);
      //      printf("Face x:%u,y:%u,z:%u\n", face->ind.x, face->ind.y,
      //      face->ind.z);
      __face++;
    }
  }
  printf("Vertices:%u\n", _vertex);
  printf("Texels:%u\n", _texture);
  printf("Normals:%u\n", _normal);
  printf("Faces:%u\n", __face);
  model->path = path;
  fclose(file);
  return model;
}
struct Buffer {
  unsigned int *data;
  unsigned int width;
  unsigned int height;
  unsigned int channels;
} typedef Buffer;
void line(int x0, int y0, int x1, int y1, Buffer *buffer) {
  int dx = abs(x0 - x1);
  int dy = abs(y0 - y1);

  int err = dx - dy;
  int err2;

  int x = x0;
  int y = y0;

  int sx = x0 < x1 ? 1 : -1;
  int sy = y0 < y1 ? 1 : -1;

  int index;
  while (x != x1 || y != y1) {
    index = (y * buffer->width + x) * buffer->channels;
    buffer->data[index] = 1;
    buffer->data[index + 1] = 2;
    buffer->data[index + 2] = 3;
  }
  err2 = 2 * err;
  if (err2 > -dy) {
    err -= dy;
    x += sx;
  }
  if (err2 < dx) {
    err += dx;
    y += sy;
  }
}
void draw(Model *model, Buffer *buffer) {
  unsigned int index, count = buffer->height * buffer->width * buffer->channels;
  for (unsigned int i = 0; i < count; i += 3) {
    unsigned int R, G, B;
    R = buffer->data[i];
    G = buffer->data[i + 1];
    B = buffer->data[i + 2];
    printf("R:%u, G:%u, B:%u\n", R, G, B);
  }
}

int main(int argc, char *argv[]) {
  const char *path = argv[1];
  unsigned int height = 1080, width = 1080, channels = 3;

  Buffer *buffer = (Buffer *)(malloc(sizeof(Buffer)));
  buffer->data =
      (unsigned int *)(malloc(sizeof(unsigned int) * height * width * 3));
  buffer->width = width;
  buffer->height = height;
  buffer->channels = channels;

  Model *model = read(path);
  draw(model, buffer);
  return 0;
}
