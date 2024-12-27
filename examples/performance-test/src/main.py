from optimizely import optimizely
from optimizely.decision.optimizely_decide_option import OptimizelyDecideOption


def datafile():
    with open('../../datafiles/sandbox.json', 'r') as file:
        return file.read()


client = optimizely.Optimizely(
    datafile=datafile(),
    default_decide_options = [
        OptimizelyDecideOption.DISABLE_DECISION_EVENT
    ]
)
flag_key = 'buy_button'

for i in range(1000000):
    user_id = 'user{}'.format(i)
    user_context = client.create_user_context(user_id)
    decision = user_context.decide(flag_key)
