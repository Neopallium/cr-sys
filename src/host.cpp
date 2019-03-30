#define CR_HOST
#include "../vendor/cr.h"

extern "C" void wrap_cr_set_temporary_path(cr_plugin &ctx, const char *path) {
	std::string str_path = path;
	cr_set_temporary_path(ctx, str_path);
}

// TODO: Remove later.  This is a temporary fix.
extern "C" void rust_cr_plugin_reload_fix(cr_plugin &ctx) {
	if(cr_plugin_changed(ctx)) {
		CR_TRACE
		// Find next unused file version.
		auto p = (cr_internal *)ctx.p;
		auto new_file = cr_version_path(p->fullname, ctx.version, p->temppath);
		while (cr_exists(new_file)) {
			ctx.version++;
			CR_LOG("file already exists '%s' bump version: %d\n", new_file.c_str(), ctx.version);
			new_file = cr_version_path(p->fullname, ctx.version, p->temppath);
		}

		// continue normal reload.
		cr_plugin_reload(ctx);
	}
}

// wrap cr_plugin_update()
extern "C" int rust_cr_plugin_update_fix(cr_plugin &ctx, bool reloadCheck = true) {
	// Use our "safe" reload function.
	if(!ctx.failure && reloadCheck) {
		rust_cr_plugin_reload_fix(ctx);
	}
	return cr_plugin_update(ctx, false);
}

