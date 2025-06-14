
import threading



def install_agents(agents):
    def install_agent(agent_name, agent):
        agent.clone_code()
        agent.install()

    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=install_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()


def start_agents(agents):
    def start_agent(agent_name, agent):
        agent.get_pids()
        agent.sync_config()
        agent.run()
    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=start_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()

def update_agent_config(agents): # 热加载
    for agent_name, agent in agents.items():
        agent.update_config()

def sync_agent_config(agents): # 冷加载
    for agent_name, agent in agents.items():
        agent.get_pids()
        agent.sync_config()

def stop_agents(agents):
    for agent_name, agent in agents.items():
        agent.stop()

def test_agents(agents):
    print("Test start")
    command = "echo 'Test Success'"  # 要在代理上执行的命令
    # 遍历所有代理并执行命令
    for agent_name, agent in agents.items():
        output, error = agent.execute_command(command)

        if output:
            print(f"{agent_name} output: {output}")
        if error:
            print(f"{agent_name} error: {error}")


