# FUSE Cloud File API PoC

I was complaining about how Linux doesn't have any sort of On-Demand Cloud File API standard, so I decided to try and make a proof of concept.

I've made some xattrs (see `src/xattrs.rs`) based on the NextCloud VFS ([link](https://github.com/nextcloud/desktop/blob/d73bd2d7e51eb9982ab152125ade47c70c5aeb01/src/common/vfs.h)) and [xdg/xdg-specs#83](https://gitlab.freedesktop.org/xdg/xdg-specs/-/issues/83).

The implementation will use FUSE IO Passthrough to act similar to an OverlayFS, except we're passing everything back to the directory underneath the mount.
The xattrs will be used to configure the sync behavior.

The PoC will either be read only or do FUSE passthrough to the mount directory. Sync Metadata (xattrs) will be stored separately.
