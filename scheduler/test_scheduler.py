import unittest
from scheduler_main import Scheduler

class TestScheduler(unittest.TestCase):

    def test_rr_quantum_must_be_positive(self):
        s = Scheduler(5)
        with self.assertRaises(AssertionError):
            s.roundrobin(quantum=0, total=100,verbose=False)

    def test_rr_time_never_exceeds_total(self):
        s = Scheduler(8)
        completed, t = s.roundrobin(quantum=5, total=100,verbose=False)
        self.assertLessEqual(t, 100)

    def test_rr_completed_processes_limit(self):
        n=-1
        s = Scheduler(n)
        completed, _ = s.roundrobin(quantum=10, total=200,verbose=False)
        self.assertLessEqual(len(completed), n)
