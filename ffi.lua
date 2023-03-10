local ffi = require("ffi")

ffi.cdef([[
typedef struct Workspace Workspace;

typedef struct FileList {
  char const* const* data;
  size_t length;
} FileList;

struct Workspace *create_workspace(const char *name, const char *path);

void destroy_files(struct FileList *files);

void destroy_workspace(struct Workspace *workspace);

struct FileList *workspace_files(const struct Workspace *workspace);
]])

local neorg_lib = ffi.load("/home/user/...path/target/release/libneorg_directory_manager.so")

local workspace = neorg_lib.create_workspace("test", "/home/user/neorg")
local files = neorg_lib.workspace_files(workspace)

local len = tonumber(files.length)
print(len)

print(ffi.string(files.data[0]))

print(ffi.string(files.data[1]))

neorg_lib.destroy_files(files)
neorg_lib.destroy_workspace(workspace)
