# Minecraft server status Tray Icon for Windows
&nbsp;
## Options

#### watch_process 
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| None | Yes  | String | The proccess that is watched
#### max_processes 
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| 2 | No  | Int | The max amount of icons that can be created, must be a positive number.

#### debug 
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| false | No  | Boolean | Print debug info.
#### main_thread_sleep_sec 
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| 15 | No  | Int | The time between check server status, must be a positive number.

#### watchers
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| None | Yes | Array | Determines which icon is associated with what server.

## Watcher properties
&nbsp;
#### type
| Default | Required | Type |  Description | Values |
| ------ | ------ | ------ | ------ | ------ |
| None | Yes | Enum | What the minecraft is. | java, bedrock 
#### ip
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| None | Yes | String | The host or ip of the server.
#### port
| Default | Required | Type |  Description | 
| ------ | ------ | ------ | ------ |
| None | Yes | Int | The port of the server. (java default is 25565) and (bedrock default is 19132)
### Example

```json
{
 "watch_process": "firefox.exe",
  "max_processes": 2,
  "main_thread_sleep_sec": 15,
  "debug": true,
  "watchers": [
    {
      "type": "java",  
      "ip": "Play.datblock.com", 
      "port": 25565 
    },
    { 
      "type": "bedrock", 
      "ip": "192.168.1.164", 
      "port": 19132 
    }
  ]
}
```
