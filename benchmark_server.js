const {cpus} = require('os')
const cluster = require('cluster')
const {createServer} = require('restify')

/**
 * Fake logger
 */
const logger = {
    info: (prefixToPrint, messageToPrint) => {console.log(`${prefixToPrint} ${messageToPrint}`)}
}

/**
 * Fork the same number of `CPU amount` worker processes
 */
const setupMaster = () => {
    const workerAmount = cpus().length

    logger.info(`setupMaster`, `Cluster worker amount: ${workerAmount}`)

    cluster.on('listening', (worker, address) => {
        logger.info(`setupMaster`, `Cluster worker "${worker.id}" (PID: ${worker.process.pid}) is listening on ${address.address}:${address.port}.`)
    })

    cluster.on('online', worker => {
        logger.info(`setupMaster`, `Cluster worker "${worker.id}" (PID: ${worker.process.pid}) is online.`)
    })

    cluster.on('disconnect', worker => {
        logger.info(`setupMaster`, `Cluster worker "${worker.id}" (PID: ${worker.process.pid}) is disconnected.`)

        // globalSession.removeAllSessionsByWorkerId(worker.id)
        // logger.debug(`cluster on disconnect`, `\n${globalSession.getSessionTable()}`)
    })

    cluster.on('exit', (worker, code, signal) => {
        logger.info(`setupMaster`, `Cluster worker "${worker.id}" (PID: ${worker.process.pid}) is exit, code=${signal || code}`)

        // We make sure launch the "missing worker" like a "self-healing" effect, then the service will NEVER die!!!
        cluster.fork()
    })

    for (let i = 0; i < workerAmount; i += 1) {
        cluster.fork()
    }
}



/**
 * 
 */
const createBenchmarkHttpServer = () => {

    const internalServer = createServer({name: 'Benchmark Http Server'})

    // Default route
    internalServer.get(`/`, (_, res, next) => {
        res.status(200)
        res.send(`Benchmark testing.`)
        next()
    })

    // JSON route
    internalServer.get(`/json-benchmark`, (_, res, next) => {
        const defaultUser = ({
            name: `Wison Ye`,
            role: `Administrator`,
            settings: {
                prefer_language: `English`,
                reload_when_changed: true,
            }
        })
        res.status(200)
        res.send(defaultUser)
        next()
    })

    /**
     * Start server
     */
    internalServer.listen(8080, '127.0.0.1', () => {
        const logPrefix = () => {return cluster.isMaster ? `Master Process | ` : `Worker Process ${cluster.worker.id} (PID: ${cluster.worker.process.pid}) | `}
        logger.info(`run`, `${logPrefix()} "${internalServer.name}" is running at ${internalServer.url}`)
    })
}

/**
 * Run cluster
 */
if (cluster.isMaster) {
    setupMaster()
} else {
    createBenchmarkHttpServer()
}
