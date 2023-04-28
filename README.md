# toodeep

For when you forget to call some generator with a `.` at the end

---

A screenplay demonstrating the issue at hand

```

FADE IN:

EXT. DEVELOPER'S COMPUTER - DAY

DEVELOPER is sitting at their desk, typing on their computer. 
TERMINAL makes a series of computer noises with each keystroke.

DEVELOPER
(annoyed)
      Why does this always happen? Every time I forget the dot, 
      it creates a new folder within the folder I'm already in.

DEVELOPER types `yarn new tauri-app` into their terminal. 
TERMINAL makes a computer noise and outputs tauri-app/tauri-app.

DEVELOPER
(getting more frustrated)
      Are you kidding me? I'm already in the tauri-app folder. 
      Now I have to move tauri-app out of tauri-app.

DEVELOPER types mv tauri-app/* ./ && mv tauri-app/.* ./ && rm -r tauri-app into 
their terminal. TERMINAL makes a series of high pitched beeps and twirps and 
outputs mv: cannot stat 'tauri-app/*': No such file or directory.

DEVELOPER
(frustrated)
      Ugh, I hate how many keystokes I'm wasting right now.

FADE OUT.

```

This is really a useless cli that you could do with 2 bash commands but i don't care I wanna learn rust! fuck you.

## Usage

```
$ pwd
/Users/tomhill/Developer/experiments/toodeep

$ ls
toodeep/

$ toodeep
moving "/Users/tomhill/Developer/experiments/toodeep/toodeep" up here...

done!
```