const $status = document.querySelector('#status')
const $form = document.querySelector('#chatform')

/** @type {WebSocket | null} */
var socket = null

function connect() {
    disconnect()

    const { location } = window

    const proto = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${proto}://${location.host}/ws`

    console.log('Connecting...')
    socket = new WebSocket(wsUri)

    socket.onopen = () => {
        console.log('Connected')
        updateConnectionStatus()
    }

    socket.onmessage = (ev) => {
        console.log('Received: ' + ev.data, 'message')
    }

    socket.onclose = () => {
        console.log('Disconnected')
        socket = null
        updateConnectionStatus()
    }
}

function disconnect() {
    if (socket) {
        console.log('Disconnecting...')
        socket.close()
        socket = null

        updateConnectionStatus()
    }
}

function updateConnectionStatus() {
    if (socket) {
        $status.style.backgroundColor = 'transparent'
        $status.style.color = 'green'
        $status.textContent = `connected`
    } else {
        $status.style.backgroundColor = 'red'
        $status.style.color = 'white'
        $status.textContent = 'disconnected'
    }
}

$connectButton.addEventListener('click', () => {
    if (socket) {
        disconnect()
    } else {
        connect()
    }

    updateConnectionStatus()
})

$form.addEventListener('submit', (ev) => {
    ev.preventDefault()

    const text = $input.value

    console.log('Sending: ' + text)
    socket.send(text)

    $input.value = ''
    $input.focus()
})

updateConnectionStatus()