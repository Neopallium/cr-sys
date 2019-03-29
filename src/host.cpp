#define CR_HOST
#include "../vendor/cr.h"

extern "C" void wrap_cr_set_temporary_path(cr_plugin &ctx, const char *path) {
	std::string str_path = path;
	cr_set_temporary_path(ctx, str_path);
}

