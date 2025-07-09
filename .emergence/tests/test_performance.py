#!/usr/bin/env python3
"""
EMERGENCE Performance Tests

This module contains performance tests that validate the performance characteristics
of the EMERGENCE autonomous testing system components.
"""

import yaml
import json
import os
import sys
import time
import psutil
import threading
from pathlib import Path
from typing import Dict, Any, List, Optional
from dataclasses import dataclass
from datetime import datetime
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class PerformanceTestResult:
    """Represents the result of a performance test"""
    test_name: str
    passed: bool
    message: str
    execution_time: float
    memory_usage: float
    cpu_usage: float
    details: Optional[Dict[str, Any]] = None

class EmergencePerformanceTestSuite:
    """Performance test suite for the EMERGENCE system"""
    
    def __init__(self, base_dir: str = ".."):
        self.base_dir = Path(base_dir)
        self.schemas: Dict[str, Any] = {}
        self.test_results: List[PerformanceTestResult] = []
        self.load_schemas()
    
    def load_schemas(self) -> None:
        """Load all schema files for performance testing"""
        logger.info("Loading EMERGENCE schemas for performance testing...")
        
        schemas_dir = self.base_dir / "schemas"
        schema_files = [
            "essences/tester-essence.yaml",
            "capabilities/test-generation.yaml",
            "capabilities/test-optimization.yaml",
            "behaviors/autonomous-testing.yaml",
            "physics/testing-environment.yaml"
        ]
        
        for schema_file in schema_files:
            file_path = schemas_dir / schema_file
            if file_path.exists():
                try:
                    with open(file_path, 'r') as f:
                        schema_data = yaml.safe_load(f)
                        schema_name = schema_file.replace('/', '_').replace('.yaml', '')
                        self.schemas[schema_name] = schema_data
                        logger.info(f"Loaded schema: {schema_name}")
                except Exception as e:
                    logger.error(f"Failed to load schema {schema_file}: {e}")
    
    def run_performance_tests(self) -> List[PerformanceTestResult]:
        """Run all performance tests"""
        logger.info("Starting EMERGENCE performance tests...")
        
        test_methods = [method for method in dir(self) if method.startswith('test_')]
        
        for test_method in test_methods:
            test_func = getattr(self, test_method)
            if callable(test_func):
                self._run_performance_test(test_method, test_func)
        
        return self.test_results
    
    def _run_performance_test(self, test_name: str, test_func) -> None:
        """Execute a single performance test"""
        start_time = time.time()
        start_memory = psutil.Process().memory_info().rss / 1024 / 1024  # MB
        start_cpu = psutil.cpu_percent(interval=0.1)
        
        try:
            result = test_func()
            execution_time = time.time() - start_time
            end_memory = psutil.Process().memory_info().rss / 1024 / 1024  # MB
            end_cpu = psutil.cpu_percent(interval=0.1)
            
            memory_usage = end_memory - start_memory
            cpu_usage = (start_cpu + end_cpu) / 2
            
            test_result = PerformanceTestResult(
                test_name=test_name,
                passed=result.get('passed', False),
                message=result.get('message', ''),
                execution_time=execution_time,
                memory_usage=memory_usage,
                cpu_usage=cpu_usage,
                details=result.get('details', {})
            )
            
            self.test_results.append(test_result)
            
            status = "PASSED" if test_result.passed else "FAILED"
            logger.info(f"{status}: {test_name} - {test_result.message} "
                       f"(Time: {execution_time:.3f}s, Memory: {memory_usage:.2f}MB, CPU: {cpu_usage:.1f}%)")
            
        except Exception as e:
            execution_time = time.time() - start_time
            memory_usage = psutil.Process().memory_info().rss / 1024 / 1024 - start_memory
            cpu_usage = psutil.cpu_percent(interval=0.1)
            
            test_result = PerformanceTestResult(
                test_name=test_name,
                passed=False,
                message=f"Performance test failed with exception: {str(e)}",
                execution_time=execution_time,
                memory_usage=memory_usage,
                cpu_usage=cpu_usage,
                details={'exception': str(e)}
            )
            self.test_results.append(test_result)
            logger.error(f"FAILED: {test_name} - Exception: {e}")
    
    def test_schema_loading_performance(self) -> Dict[str, Any]:
        """Test performance of schema loading"""
        start_time = time.time()
        
        # Simulate loading multiple schemas
        loaded_schemas = 0
        for _ in range(10):  # Load schemas multiple times to test performance
            for schema_name, schema_data in self.schemas.items():
                # Simulate schema processing
                if isinstance(schema_data, dict):
                    loaded_schemas += 1
                    # Simulate some processing time
                    time.sleep(0.001)
        
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 1.0  # 1 second
        max_memory_usage = 50.0   # 50 MB
        
        passed = execution_time <= max_execution_time
        
        return {
            'passed': passed,
            'message': f"Schema loading performance: {execution_time:.3f}s for {loaded_schemas} schemas",
            'details': {
                'execution_time': execution_time,
                'loaded_schemas': loaded_schemas,
                'max_execution_time': max_execution_time,
                'performance_ratio': execution_time / max_execution_time
            }
        }
    
    def test_essence_processing_performance(self) -> Dict[str, Any]:
        """Test performance of essence processing"""
        start_time = time.time()
        
        essence_schema = self.schemas.get('essences_tester-essence')
        if not essence_schema:
            return {
                'passed': False,
                'message': "Essence schema not found",
                'details': {}
            }
        
        # Simulate essence processing operations
        processing_operations = 0
        
        # Process personality traits
        personality = essence_schema.get('personality', {})
        for trait, value in personality.items():
            processing_operations += 1
            # Simulate trait validation
            if not isinstance(value, (int, float)) or value < 0 or value > 1:
                pass  # Invalid trait
        
        # Process capabilities
        capabilities = essence_schema.get('capabilities', {})
        for capability_type, capability_list in capabilities.items():
            if isinstance(capability_list, list):
                processing_operations += len(capability_list)
        
        # Process behavioral patterns
        behavioral_patterns = essence_schema.get('behavioral_patterns', {})
        for pattern_name, pattern_data in behavioral_patterns.items():
            processing_operations += 1
            # Simulate pattern analysis
            time.sleep(0.001)
        
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 0.5  # 500ms
        max_operations = 1000
        
        passed = execution_time <= max_execution_time and processing_operations <= max_operations
        
        return {
            'passed': passed,
            'message': f"Essence processing: {execution_time:.3f}s for {processing_operations} operations",
            'details': {
                'execution_time': execution_time,
                'processing_operations': processing_operations,
                'max_execution_time': max_execution_time,
                'max_operations': max_operations
            }
        }
    
    def test_capability_processing_performance(self) -> Dict[str, Any]:
        """Test performance of capability processing"""
        start_time = time.time()
        
        capability_schemas = [
            self.schemas.get('capabilities_test-generation'),
            self.schemas.get('capabilities_test-optimization')
        ]
        
        processing_operations = 0
        
        for schema in capability_schemas:
            if not schema:
                continue
            
            # Process core functions
            core_functions = schema.get('core_functions', {})
            for func_name, func_data in core_functions.items():
                processing_operations += 1
                # Simulate function analysis
                time.sleep(0.001)
            
            # Process learning mechanisms
            learning_mechanisms = schema.get('learning_mechanisms', {})
            for mechanism_name, mechanism_data in learning_mechanisms.items():
                processing_operations += 1
                # Simulate mechanism analysis
                time.sleep(0.001)
            
            # Process emergence potentials
            emergence_potentials = schema.get('emergence_potentials', {})
            for potential_name, potential_data in emergence_potentials.items():
                processing_operations += 1
                # Simulate potential analysis
                time.sleep(0.001)
        
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 1.0  # 1 second
        max_operations = 2000
        
        passed = execution_time <= max_execution_time and processing_operations <= max_operations
        
        return {
            'passed': passed,
            'message': f"Capability processing: {execution_time:.3f}s for {processing_operations} operations",
            'details': {
                'execution_time': execution_time,
                'processing_operations': processing_operations,
                'max_execution_time': max_execution_time,
                'max_operations': max_operations
            }
        }
    
    def test_behavior_processing_performance(self) -> Dict[str, Any]:
        """Test performance of behavior processing"""
        start_time = time.time()
        
        behavior_schema = self.schemas.get('behaviors_autonomous-testing')
        if not behavior_schema:
            return {
                'passed': False,
                'message': "Behavior schema not found",
                'details': {}
            }
        
        processing_operations = 0
        
        # Process behavior sequence
        behavior_sequence = behavior_schema.get('behavior_sequence', {})
        for phase_name, phase_data in behavior_sequence.items():
            processing_operations += 1
            # Simulate phase analysis
            time.sleep(0.002)
        
        # Process learning integration
        learning_integration = behavior_schema.get('learning_integration', {})
        for learning_type, learning_data in learning_integration.items():
            processing_operations += 1
            # Simulate learning analysis
            time.sleep(0.001)
        
        # Process emergence potentials
        emergence_potentials = behavior_schema.get('emergence_potentials', {})
        for potential_name, potential_data in emergence_potentials.items():
            processing_operations += 1
            # Simulate potential analysis
            time.sleep(0.001)
        
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 0.8  # 800ms
        max_operations = 1500
        
        passed = execution_time <= max_execution_time and processing_operations <= max_operations
        
        return {
            'passed': passed,
            'message': f"Behavior processing: {execution_time:.3f}s for {processing_operations} operations",
            'details': {
                'execution_time': execution_time,
                'processing_operations': processing_operations,
                'max_execution_time': max_execution_time,
                'max_operations': max_operations
            }
        }
    
    def test_physics_processing_performance(self) -> Dict[str, Any]:
        """Test performance of physics processing"""
        start_time = time.time()
        
        physics_schema = self.schemas.get('physics_testing-environment')
        if not physics_schema:
            return {
                'passed': False,
                'message': "Physics schema not found",
                'details': {}
            }
        
        processing_operations = 0
        
        # Process environment properties
        environment_properties = physics_schema.get('environment_properties', {})
        for prop_name, prop_data in environment_properties.items():
            processing_operations += 1
            # Simulate property analysis
            time.sleep(0.001)
        
        # Process forces
        forces = physics_schema.get('forces', {})
        for force_name, force_data in forces.items():
            processing_operations += 1
            # Simulate force analysis
            time.sleep(0.002)
        
        # Process interactions
        interactions = physics_schema.get('interactions', {})
        for interaction_name, interaction_data in interactions.items():
            processing_operations += 1
            # Simulate interaction analysis
            time.sleep(0.001)
        
        # Process energy dynamics
        energy_dynamics = physics_schema.get('energy_dynamics', {})
        for energy_type, energy_data in energy_dynamics.items():
            processing_operations += 1
            # Simulate energy analysis
            time.sleep(0.001)
        
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 1.2  # 1.2 seconds
        max_operations = 2500
        
        passed = execution_time <= max_execution_time and processing_operations <= max_operations
        
        return {
            'passed': passed,
            'message': f"Physics processing: {execution_time:.3f}s for {processing_operations} operations",
            'details': {
                'execution_time': execution_time,
                'processing_operations': processing_operations,
                'max_execution_time': max_execution_time,
                'max_operations': max_operations
            }
        }
    
    def test_memory_efficiency(self) -> Dict[str, Any]:
        """Test memory efficiency of the system"""
        start_memory = psutil.Process().memory_info().rss / 1024 / 1024  # MB
        
        # Simulate memory-intensive operations
        large_data_structures = []
        for i in range(1000):
            large_data_structures.append({
                'schema_name': f'schema_{i}',
                'data': {f'key_{j}': f'value_{j}' for j in range(100)}
            })
        
        # Process schemas multiple times
        for _ in range(5):
            for schema_name, schema_data in self.schemas.items():
                # Simulate deep processing
                if isinstance(schema_data, dict):
                    self._deep_process_schema(schema_data)
        
        end_memory = psutil.Process().memory_info().rss / 1024 / 1024  # MB
        memory_increase = end_memory - start_memory
        
        # Performance thresholds
        max_memory_increase = 100.0  # 100 MB
        max_memory_usage = 500.0     # 500 MB total
        
        passed = memory_increase <= max_memory_increase and end_memory <= max_memory_usage
        
        return {
            'passed': passed,
            'message': f"Memory efficiency: {memory_increase:.2f}MB increase, {end_memory:.2f}MB total",
            'details': {
                'memory_increase': memory_increase,
                'total_memory': end_memory,
                'max_memory_increase': max_memory_increase,
                'max_memory_usage': max_memory_usage
            }
        }
    
    def _deep_process_schema(self, schema_data: Dict[str, Any], depth: int = 0) -> None:
        """Recursively process schema data to simulate deep processing"""
        if depth > 5:  # Prevent infinite recursion
            return
        
        for key, value in schema_data.items():
            if isinstance(value, dict):
                self._deep_process_schema(value, depth + 1)
            elif isinstance(value, list):
                for item in value:
                    if isinstance(item, dict):
                        self._deep_process_schema(item, depth + 1)
    
    def test_concurrent_processing(self) -> Dict[str, Any]:
        """Test concurrent processing performance"""
        start_time = time.time()
        
        def process_schema_worker(schema_name: str, schema_data: Dict[str, Any]) -> int:
            """Worker function to process a schema"""
            operations = 0
            if isinstance(schema_data, dict):
                for key, value in schema_data.items():
                    operations += 1
                    # Simulate processing time
                    time.sleep(0.001)
            return operations
        
        # Create threads for concurrent processing
        threads = []
        results = []
        
        for schema_name, schema_data in self.schemas.items():
            thread = threading.Thread(
                target=lambda: results.append(process_schema_worker(schema_name, schema_data))
            )
            threads.append(thread)
            thread.start()
        
        # Wait for all threads to complete
        for thread in threads:
            thread.join()
        
        total_operations = sum(results)
        execution_time = time.time() - start_time
        
        # Performance thresholds
        max_execution_time = 2.0  # 2 seconds for concurrent processing
        min_operations = 100      # Minimum operations expected
        
        passed = execution_time <= max_execution_time and total_operations >= min_operations
        
        return {
            'passed': passed,
            'message': f"Concurrent processing: {execution_time:.3f}s for {total_operations} operations",
            'details': {
                'execution_time': execution_time,
                'total_operations': total_operations,
                'threads_used': len(threads),
                'max_execution_time': max_execution_time,
                'min_operations': min_operations
            }
        }
    
    def generate_performance_report(self) -> Dict[str, Any]:
        """Generate a comprehensive performance test report"""
        total_tests = len(self.test_results)
        passed_tests = sum(1 for result in self.test_results if result.passed)
        failed_tests = total_tests - passed_tests
        
        total_execution_time = sum(result.execution_time for result in self.test_results)
        total_memory_usage = sum(result.memory_usage for result in self.test_results)
        avg_cpu_usage = sum(result.cpu_usage for result in self.test_results) / total_tests if total_tests > 0 else 0
        
        report = {
            'summary': {
                'total_tests': total_tests,
                'passed_tests': passed_tests,
                'failed_tests': failed_tests,
                'success_rate': passed_tests / total_tests if total_tests > 0 else 0,
                'total_execution_time': total_execution_time,
                'total_memory_usage': total_memory_usage,
                'average_cpu_usage': avg_cpu_usage
            },
            'test_results': [
                {
                    'test_name': result.test_name,
                    'passed': result.passed,
                    'message': result.message,
                    'execution_time': result.execution_time,
                    'memory_usage': result.memory_usage,
                    'cpu_usage': result.cpu_usage,
                    'details': result.details
                }
                for result in self.test_results
            ],
            'timestamp': datetime.now().isoformat()
        }
        
        return report

def main():
    """Main function to run performance tests"""
    print("EMERGENCE Performance Test Suite")
    print("=" * 40)
    
    # Initialize performance test suite
    test_suite = EmergencePerformanceTestSuite()
    
    # Run all performance tests
    results = test_suite.run_performance_tests()
    
    # Generate report
    report = test_suite.generate_performance_report()
    
    # Print summary
    print(f"\nPerformance Test Summary:")
    print(f"Total Tests: {report['summary']['total_tests']}")
    print(f"Passed: {report['summary']['passed_tests']}")
    print(f"Failed: {report['summary']['failed_tests']}")
    print(f"Success Rate: {report['summary']['success_rate']:.2%}")
    print(f"Total Execution Time: {report['summary']['total_execution_time']:.3f}s")
    print(f"Total Memory Usage: {report['summary']['total_memory_usage']:.2f}MB")
    print(f"Average CPU Usage: {report['summary']['average_cpu_usage']:.1f}%")
    
    # Print detailed results
    print(f"\nDetailed Performance Results:")
    for result in report['test_results']:
        status = "✓ PASS" if result['passed'] else "✗ FAIL"
        print(f"{status}: {result['test_name']} - {result['message']}")
    
    # Save report to file
    report_file = ".emergence/performance_test_report.json"
    os.makedirs(os.path.dirname(report_file), exist_ok=True)
    
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2)
    
    print(f"\nPerformance report saved to: {report_file}")
    
    # Exit with appropriate code
    if report['summary']['failed_tests'] > 0:
        print("\n❌ Some performance tests failed!")
        sys.exit(1)
    else:
        print("\n✅ All performance tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main() 