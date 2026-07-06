__version__= "0.4"
import random as r
import argparse
from pathlib import Path

READY=0b00
RUNNING=0b01
WAITING=0b10
DONE=0b11

STATES={READY:"READY",RUNNING:"RUNNING",WAITING:"WAITING",DONE:"DONE"}
class Process:
    def __init__(self,verbose=False):
        """Initializes processes and assigns them a process ID."""
        self.pid=r.randint(1,1000)
        if verbose:
            print("Assigned a process with pid",self.pid)

class Scheduler():
    """Runs scheduler algorithms as per call"""
    def __init__(self,p,log_file=None):
        if type(p) not in [int] or p<=0:
            raise ValueError("Process count must be greater than 0 and a natural number")
        self.plist=[]
        self.state={}
        self.log_file=log_file
        for _ in range(p):
            task=Process()
            self.plist.append(task.pid)
            self.state[task.pid]=READY
    def setstate(self,pid,newstate,autotrace):
        oldstate=self.state[pid]
        if oldstate==newstate:
            return
        self.state[pid]=newstate
        
        if autotrace:
            msg = f"[TRACE] PID {pid}: {STATES[oldstate]} -> {STATES[newstate]}"
            print(f"[TRACE] PID {pid}: " f"{STATES[oldstate]} -> {STATES[newstate]}")
            
            if self.log_file:
                with self.log_file.open("a") as f:
                    f.write(msg + "\n")
    
    def fcfs(self,verbose=False,autotrace=False):
        """Runs the non preemptive version of the FCFS scheduler algorithm"""
        for i in range(len(self.plist)):
            self.setstate(self.plist[i],RUNNING,autotrace)
            if verbose:
                print("Process with pid",self.plist[i],"started")
            t=r.randint(1,5)
            while(t>0):
                t-=1
                if verbose:
                    print("Running...")
            self.setstate(self.plist[i],DONE,autotrace)
            if verbose:
                print("Process finished!")
        if verbose:
            print("Scheduler completed all generated processes") 
    
    def priorityschedule(self,verbose=False,autotrace=False):
        """Uses the priority scheduling algorithm"""
        #Assign random priority for now since this is a simulation
        priority=[(pid,r.randint(1,len(self.plist))) for pid in self.plist]
        priority.sort(key=lambda x: x[1])  # Descending priority
        for pid,_ in priority:
            self.setstate(pid,RUNNING,autotrace)
            if verbose:
                print("Running")
            self.setstate(pid,DONE,autotrace)

    def roundrobin(self,quantum,total,verbose=False,autotrace=False):
        """Simulates Round Robin with quantum and total algorithm time"""
        assert quantum>0,"Quantum must be greater than 0"
        assert total>0,"Total scheduler time should be greater than 0"
        q=[(pid,r.randint(1,15)) for pid in self.plist]
        t=0 
        c=[]
        while q:
            pid,remain=q.pop(0)
            self.setstate(pid,RUNNING,autotrace)
            assert remain>0
            run_time=min(quantum,remain)
            remain-=run_time
            t+=run_time
            assert t>=0
            if verbose:
                print("Process with pid",pid,"running for",run_time,"seconds")
            if remain>0:
                self.setstate(pid,WAITING,autotrace)
                q.append((pid,remain))
            else:
                self.setstate(pid,DONE,autotrace)
                if verbose:
                    print("Process with id",pid,"completed")
                c.append(pid)
            
            if t>=total:
                break
        if verbose:
            print("Scheduler completed the assigned tasks")
            print("Total scheduler time used : t=",t)
        assert len(c)<=len(self.plist)
        return c,t
    
    def shortest_job_first(self,verbose=False,autotrace=False):

        ready_queue = [(pid,r.randint(1,15)) for pid in self.plist]
        ready_queue.sort(key=lambda x: x[1])
        current_time = 0
        for pid, burst in ready_queue:
            start_time = current_time
            end_time = current_time + burst
            if verbose:
                print(f"Process {pid} started at time {start_time} and will run for {burst} units")
            self.setstate(pid,RUNNING,autotrace)
            if verbose:
                print(f"Process {pid} completed at time {end_time}")
            self.setstate(pid,DONE,autotrace)
        
            current_time = end_time

        print(f"All processes completed. Total time: {current_time}")
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=f"Python Scheduler Simulator v{__version__}")
    parser.add_argument("--version",action="version",version=f"Python Scheduler Simulator v{__version__}")
    parser.add_argument("-p", "--processes", type=int, default=5,
                        help="Number of processes (default: %(default)s)")
    parser.add_argument("-a", "--algorithm", choices=["fcfs", "priority", "rr","sjf"],
                        default="rr", help="Scheduler algorithm to use (default: %(default)s)")
    parser.add_argument("-q", "--quantum", type=int, default=5,
                        help="Quantum for Round Robin (default: %(default)s)")
    parser.add_argument("-t", "--total", type=int, default=50,
                        help="Total scheduler time (default: %(default)s)")
    parser.add_argument("-v", "--verbose", action="store_true",
                        help="Enable verbose output")
    parser.add_argument("-A","--autotrace",action="store_true",help="See tracing details")
    parser.add_argument("-r","--route",type=str,help="Move results of the scheduler to a user-specified file")
    
    args = parser.parse_args()

    log_file = None
    if args.route:
        log_file = Path(args.route)
        if not log_file.exists():
            log_file.parent.mkdir(parents=True, exist_ok=True) # create folders if missing
            log_file.touch()
    print(f"[INFO] Logging states to {log_file}")

    scheduler = Scheduler(args.processes,log_file=log_file)
    if args.algorithm == "fcfs":
        scheduler.fcfs(verbose=args.verbose,autotrace=args.autotrace)
    elif args.algorithm == "priority":
        scheduler.priorityschedule(verbose=args.verbose,autotrace=args.autotrace)
    elif args.algorithm == "rr":
        scheduler.roundrobin(quantum=args.quantum, total=args.total, verbose=args.verbose,autotrace=args.autotrace)
    else:
        scheduler.shortest_job_first(verbose=args.verbose,autotrace=args.autotrace)