import argparse
from controller.src.utils import *
from config.parse_config import load_agents
from database.src.utils import es_read_agent_span_list, es_clear_all
from trace.association.src.cross import inter_association
from trace.association.src import fifo, deeptrace, traceweaver_v1, traceweaver_v2, wap5, vpath
from trace.association.src.utils import span_merge, print_acc
from database.src.utils import es_write_span_list
from trace.assemble.src.utils import assemble_trace_from_db
from service.src.metric import service_metrics
from callgraph.src.graph import construct_graph

from database.test.database import es_init_test_data
from trace.association.test.mix_test import perform_association
from trace.assemble.test.test import perform_assemble



def parse_args():
    parser = argparse.ArgumentParser(description='参数解析示例')
    subparsers = parser.add_subparsers(dest='command', help='可用命令', required=True)

    # agent 子命令
    agent_parser = subparsers.add_parser('agent', help='agent相关操作')
    agent_subparsers = agent_parser.add_subparsers(dest='agent_action', help='agent操作', required=True)
    agent_subparsers.add_parser('install', help='安装agent')
    agent_subparsers.add_parser('test', help='测试连接agent')
    agent_subparsers.add_parser('stop', help='停止agent')
    agent_subparsers.add_parser('run', help='运行agent')
    agent_subparsers.add_parser('sync', help='将配置同步到agent')
    agent_subparsers.add_parser('install_app', help='安装workload')  
    agent_subparsers.add_parser('uninstall_app', help='卸载workload')  # 

    # asso 子命令
    asso_parser = subparsers.add_parser('asso', help='asso相关操作')
    asso_subparsers = asso_parser.add_subparsers(dest='asso_action', help='asso操作', required=True)
    asso_algo = asso_subparsers.add_parser('algo', help='算法选择')
    asso_algo.add_argument('algorithm', choices=['fifo', 'deeptrace'], help='选择算法')

    # database 子命令
    db_parser = subparsers.add_parser('db', help='database操作')
    db_subparsers = db_parser.add_subparsers(dest='db_action', help='database操作', required=True)
    db_subparsers.add_parser('clear', help='清除所有表格')

    # trace子命令
    trace_parser = subparsers.add_parser('trace', help='trace相关操作')
    trace_subparsers = trace_parser.add_subparsers(dest='action', help='trace操作', required=True)
    asso_algo = trace_subparsers.add_parser('test', help='测试span关联和trace组装')
    asso_algo.add_argument('algorithm', choices=['fifo', 'deeptrace', 'vpath', 'wap5', 'traceweaver_v1', 'traceweaver_v2'], help='选择算法')

    # assemble 子命令
    subparsers.add_parser('assemble', help='assemble操作')
    
    # service 子命令
    service_parser = subparsers.add_parser('service', help='service相关操作')
    service_subparsers = service_parser.add_subparsers(dest='service_action', help='service操作', required=True)
    service_subparsers.add_parser('metrics', help='获取服务指标')
    
    # 添加构建图的命令
    graph_parser = subparsers.add_parser('graph', help='构建调用图')
    graph_subparsers = graph_parser.add_subparsers(dest='graph_action', help='图操作', required=True)
    graph_subparsers.add_parser('construct', help='构建调用图')
    

    return parser.parse_args()

def main():
    args = parse_args()
    if args.command == 'agent':
        agents = load_agents()
        if args.agent_action == 'install':
            install_agents(agents)
        elif args.agent_action == 'stop':
            stop_agents(agents)
        elif args.agent_action == 'test':
            print("执行agent测试操作")
            test_agents(agents)
        elif args.agent_action == 'run':
            print("执行agent运行操作")
            start_agents(agents)
        elif args.agent_action == 'sync':
            print("执行agent配置同步操作")
            sync_agent_config(agents)
        elif args.agent_action == 'install_app':
            print("执行agent工作负载安装操作")
            install_workload(agents)
        elif args.agent_action == 'uninstall_app':
            print("执行agent工作负载卸载操作")
            uninstall_workload(agents)
    elif args.command == 'asso':
        agents = load_agents()
        spans = es_read_agent_span_list(agents)
        spans = inter_association(spans, client_ingress='ComposePost', tuple_used=False)
        span_dict = {}
        if args.asso_action == 'algo':
            print(f"选择asso算法: {args.algorithm}")
            if args.algorithm == 'fifo':
                span_dict = fifo.fifo(spans)
            elif args.algorithm == 'deeptrace':
                span_dict = deeptrace.deeptrace(spans)
            elif args.algorithm == 'vpath':
                span_dict = vpath.vpath(spans)
            elif args.algorithm == 'wap5':
                span_dict = wap5.wap5(spans)
            elif args.algorithm == 'traceweaver_v1':
                span_dict = traceweaver_v1.traceweaver_v1(spans)
            elif args.algorithm == 'traceweaver_v2':
                span_dict = traceweaver_v2.traceweaver_v2(spans)
            else:
                raise ValueError(f"Unknown algorithm: {args.algorithm}")
            print_acc(span_dict)
        span_list = span_merge(span_dict)
        # fp = open('span-mappings.txt', 'w')
        # for span in span_list:
        #     if span.direction == 'Ingress':
        #         continue
        #     fp.write(f"{span.direction} {span.endpoint} {span.span_id} {span.trace_id} {span.parent_id} {span.parent_traceid}\n")
        # fp.close()
        es_write_span_list(f'span-mappings', span_list)
    elif args.command == 'assemble':
        print("执行assemble操作")
        assemble_trace_from_db('span-mappings', 'traces')
    elif args.command == 'db':
        if args.db_action ==  'clear':
            print("清除所有数据库表格")
            es_clear_all()
        
    elif args.command == 'trace':
        if args.action == 'test':
            es_init_test_data()
            perform_association(args.algorithm)
            perform_assemble()
    
    elif args.command == 'service':
        if args.service_action == 'metrics':
            print("获取服务指标")
            service_metrics()
            
    elif args.command == 'graph':
        if args.graph_action == 'construct':
            print("构建调用图")
            construct_graph()


            

if __name__ == '__main__':
    main()