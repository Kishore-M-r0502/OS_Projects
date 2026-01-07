import random as r
import argparse

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
    def __init__(self,p):
        if type(p) not in [int] or p<=0:
            raise ValueError("Process count must be greater than 0 and a natural number")
        self.plist=[]
        self.state={}
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
            print(f"[TRACE] PID {pid}: " f"{STATES[oldstate]} -> {STATES[newstate]}")
    
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
        t=0 #Counter(Scheduler)
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
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Python Scheduler Simulation")
    parser.add_argument("-p", "--processes", type=int, default=5,
                        help="Number of processes (default: %(default)s)")
    parser.add_argument("-a", "--algorithm", choices=["fcfs", "priority", "rr"],
                        default="rr", help="Scheduler algorithm to use (default: %(default)s)")
    parser.add_argument("-q", "--quantum", type=int, default=5,
                        help="Quantum for Round Robin (default: %(default)s)")
    parser.add_argument("-t", "--total", type=int, default=50,
                        help="Total scheduler time (default: %(default)s)")
    parser.add_argument("-v", "--verbose", action="store_true",
                        help="Enable verbose output")
    parser.add_argument("-A","--autotrace",action="store_true",help="See tracing details")
    
    args = parser.parse_args()

    scheduler = Scheduler(args.processes)
    if args.algorithm == "fcfs":
        scheduler.fcfs(verbose=args.verbose,autotrace=args.autotrace)
    elif args.algorithm == "priority":
        scheduler.priorityschedule(verbose=args.verbose,autotrace=args.autotrace)
    else:
        scheduler.roundrobin(quantum=args.quantum, total=args.total, verbose=args.verbose,autotrace=args.autotrace)