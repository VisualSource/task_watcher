#!/usr/bin/env node
const yargs = require('yargs/yargs')
const { hideBin } = require('yargs/helpers')
const util = require('minecraft-server-util');

try {
    yargs(hideBin(process.argv))
    .command("* <host> [port] [timeout] [srv]","ping a server",
    (opt)=>{
        return opt.positional('port',{describe: "server port", default: 19132})
        .positional('timeout',{default: 5000, describe:"How long to wait for a responce."})
        .positional('srv',{default: true, describe:""})
    },(argv)=>{
        let timeout;
        getStatus(argv).then(()=>{clearTimeout(timeout)});
        if(argv.d) console.log(argv);
        timeout = setTimeout(()=>{
            console.log("Error: failed to ping:",argv.host);
            process.exit(1);
        },argv.timeout);
    })
    .option("p",{describe: "Pritty print", default: false, type: "boolean" })
    .option("d",{describe: "Debug print", default: false, type: "boolean" })
    .argv;
} catch (error) {
    console.error(error);
}

async function getStatus(argv){
    try {
        const status = await util.statusBedrock(argv.host,{port: argv.port, enableSRV: argv.srv, timeout: argv.timeout });
        if(argv.p){
            console.log(`Host: ${status.host}:\x1b[32m${status.port}\x1b[0m`);
            console.log(`MOTD:\n`);
            console.log(status.motdLine1?.toANSI() ?? "");
            console.log(status.motdLine2?.toANSI() ?? "");
            console.log(`\nVersion: ${status.edition} \x1b[0m\x1b[35m${status.version}\x1b[0m`);
            console.log(`${status.gameMode}: \x1b[36m${status.onlinePlayers}\x1b[0m/\x1b[36m${status.maxPlayers}\x1b[0m`);
            console.log(`Protocal Version: \x1b[36m${status.protocolVersion}\x1b[0m`);
            console.log(`Latency: \x1b[33m${status.roundTripLatency}\x1b[0mms`);
        }else{
            console.log(JSON.stringify(status,(key,value)=>{
                if(typeof value === "bigint"){
                    return Number(value);
                }else{
                    return value;
                }
                ;
            },2));
        }
    } catch (error) {
        console.error(error);
    }
}