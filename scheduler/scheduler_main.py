import random as r
import argparse
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
        for _ in range(p):
            task=Process()
            self.plist.append(task.pid)
    
    def fcfs(self,verbose=False):
        """Runs the non preemptive version of the FCFS scheduler algorithm"""
        for i in range(len(self.plist)):
            if verbose:
                print("Process with pid",self.plist[i],"started")
            t=r.randint(1,5)
            while(t>0):
                if verbose:
                    print("Running...")
                t-=1
            if verbose:
                print("Process finished!")
        if verbose:
            print("Scheduler completed all generated processes") 
    
    def priorityschedule(self,verbose=False):
        """Uses the priority scheduling algorithm"""
        #Assign random priority for now since this is a simulation
        priority=[(pid,r.randint(1,len(self.plist))) for pid in self.plist]
        priority.sort(key=lambda x: x[1])  # Descending priority
        if verbose:
            for pid in priority:
                print("Process", pid, "started")
                print("Running...")
    
    def roundrobin(self,quantum,total,verbose=False):
        """Simulates Round Robin with quantum and total algorithm time"""
        assert quantum>0,"Quantum must be greater than 0"
        assert total>0,"Total scheduler time should be greater than 0"
        q=[(pid,r.randint(1,15)) for pid in self.plist]
        t=0 #Counter(Scheduler)
        c=[]
        while q:
            pid,remain=q.pop(0)
            assert remain>0
            run_time=min(quantum,remain)
            remain-=run_time
            t+=run_time
            assert t>=0
            if verbose:
                print("Process with pid",pid,"running for",run_time,"seconds")
            if remain>0:
                q.append((pid,remain))
            else:
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
    
    args = parser.parse_args()

    scheduler = Scheduler(args.processes)
    if args.algorithm == "fcfs":
        scheduler.fcfs(verbose=args.verbose)
    elif args.algorithm == "priority":
        scheduler.priorityschedule(verbose=args.verbose)
    else:
        scheduler.roundrobin(quantum=args.quantum, total=args.total, verbose=args.verbose)