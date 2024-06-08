<script>

    import { invoke } from '@tauri-apps/api/tauri'

    let clock_button_text = "Clock In";
    let clocked_in = false;

    function clock(){
        if(clocked_in){
            clock_out();
        } else {
            clock_in();
        }
    }

    function clock_in(){
        clock_button_text = "Clock Out";
        clocked_in = true;
        invoke('clock_in');
    }

    function clock_out(){
        clock_button_text = "Clock In";
        clocked_in = false;
        invoke('clock_out');
    }


</script>

<style>
    body {
        justify-content: center;
        align-items: center;
        display: flex;
        width: 100vw;
        height: 100vh;
        padding: 0;
        margin: 0;
    }
</style>
<body>
    <button on:click={clock}>{clock_button_text}</button>
</body>