const optimizely = require('@optimizely/optimizely-sdk');
const fs = require('fs');

const datafile = fs.readFileSync('../../datafiles/sandbox.json', 'utf8');

const client = optimizely.createInstance({
    datafile,
    defaultDecideOptions: [
        optimizely.OptimizelyDecideOption.DISABLE_DECISION_EVENT
    ]
});

const flag_key = 'buy_button';

for (let i = 0; i < 1000000; i++) {
    const user_id = `user${i}`;
    const user_context = client.createUserContext(user_id);
    const decision = user_context.decide(flag_key);
}
