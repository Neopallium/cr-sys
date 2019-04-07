#define CR_HOST
#include "../vendor/cr.h"

extern "C" void wrap_cr_set_temporary_path(cr_plugin &ctx, const char *path) {
	std::string str_path = path;
	cr_set_temporary_path(ctx, str_path);
}

// Return the current library filename.
extern "C" size_t rust_cr_plugin_get_filename(cr_plugin &ctx, uint8_t *buf, size_t buf_len) {
	const auto p = (cr_internal *)ctx.p;
	auto version = ctx.version > 0 ? ctx.version - 1 : 0;
	const auto file = cr_version_path(p->fullname, version, p->temppath);
	auto len = file.copy((char *)buf, buf_len);
	buf[len] = '\0';
	return len;
}

