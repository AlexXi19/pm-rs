# pm-rs
Process manager in rust

Easily start and stop processes like kubectl/ssh port forwards in the background. 

```
pmrs 0.1.0
A rust process manager to start and manage processes

USAGE:
    pm-rs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create
    help       Prints this message or the help of the given subcommand(s)
    list
    remove
    restart
    start
    stop
```

Example Usage: 
```
 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs create sleep "sleep 20"


 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs list

+-------+-------+--------+---------------------------------+----------+
| Name  | PID   | Status | Uptime                          | Command  |
+-------+-------+--------+---------------------------------+----------+
| sleep | 27132 | Active | 1 second, 618 ms and 766 µs ago | sleep 20 |
+-------+-------+--------+---------------------------------+----------+

 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs stop sleep


 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs list

+-------+----------+----------+----------+----------+
| Name  | PID      | Status   | Uptime   | Command  |
+-------+----------+----------+----------+----------+
| sleep | Inactive | Inactive | Inactive | sleep 20 |
+-------+----------+----------+----------+----------+

 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs start sleep


 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs list

+-------+-------+--------+---------------------------------+----------+
| Name  | PID   | Status | Uptime                          | Command  |
+-------+-------+--------+---------------------------------+----------+
| sleep | 27252 | Active | 1 second, 255 ms and 742 µs ago | sleep 20 |
+-------+-------+--------+---------------------------------+----------+

```
