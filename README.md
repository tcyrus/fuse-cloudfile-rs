# FUSE Cloud File API PoC

I was complaining about how Linux doesn't have any sort of On-Demand Cloud File API standard, so I decided to try and make a proof of concept.

I've made some xattrs (see `src/xattrs.rs`) based on the NextCloud VFS ([link](https://github.com/nextcloud/desktop/blob/d73bd2d7e51eb9982ab152125ade47c70c5aeb01/src/common/vfs.h)) and [xdg/xdg-specs#83](https://gitlab.freedesktop.org/xdg/xdg-specs/-/issues/83).

The implementation will use FUSE IO Passthrough to act like an OverlayFS for `$XDG_DATA_HOME/fuse-cloudfile-rs/drive/`.
The xattrs will be used to configure the sync behavior.

Because I don't want to deal with complicated behavior, I will keep the PoC as a read only fs (except for xattrs).

I haven't figured out how to expose merge conflicts via xattrs yet.
