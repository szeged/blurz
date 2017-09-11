use dbus;

error_chain!{
    foreign_links {
        DBus(dbus::Error);
    }
}
