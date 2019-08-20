import loadWasm from './lib.rs';


loadWasm().then(result => {
    const {eval_math} = result.instance.exports;
    console.log('I am alive!!!');

    const res = eval_math("283.869575/0.886558")
    console.log(res)
});
