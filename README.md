# pm-rs
Process manager in rust

Easily start and stop processes like kubectl/ssh port forwards in the background. 

```
pmrs 0.1.0
A rust process manager to start and manage processes

USAGE:
    pmrs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create     Creates and starts a new process with name
    help       Prints this message or the help of the given subcommand(s)
    list       Lists all processes being managed
    remove     Removes an inactive process with name
    restart    Restarts an active process with name
    start      Starts an inactive process with name
    stop       Stops an active process with name
```

Install:

`cargo install --git https://github.com/AlexXi19/pm-rs`

Example Usage: 
```
 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs create pf 'kubectl port-forward dev 5303:5303'


 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs list

+------+-------+--------+---------------------------------+------------------------------------+
| Name | PID   | Status | Uptime                          | Command                            |
+------+-------+--------+---------------------------------+------------------------------------+
| pf   | 24655 | Active | 1 second, 571 ms and 264 µs ago | kubectl port-forward dev 5303:5303 |
+------+-------+--------+---------------------------------+------------------------------------+

 ~/Desktop/CS/pm-rs git:(main)

 ➜ curl localhost:5303/ping

pong

 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs stop pf


 ~/Desktop/CS/pm-rs git:(main)

 ➜ pmrs list

+------+----------+----------+----------+------------------------------------+
| Name | PID      | Status   | Uptime   | Command                            |
+------+----------+----------+----------+------------------------------------+
| pf   | Inactive | Inactive | Inactive | kubectl port-forward dev 5303:5303 |
+------+----------+----------+----------+------------------------------------+

 ~/Desktop/CS/pm-rs git:(main)

 ➜ curl localhost:5303/ping

curl: (7) Failed to connect to localhost port 5303 after 5 ms: Connection refused
```
