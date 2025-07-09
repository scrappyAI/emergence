#!/usr/bin/env python3
"""
EMERGENCE Integration Tests

This module contains integration tests that verify how all EMERGENCE components
work together as a complete autonomous testing system.
"""

import yaml
import json
import os
import sys
from pathlib import Path
from typing import Dict, Any, List, Optional
from dataclasses import dataclass
from datetime import datetime
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class IntegrationTestResult:
    """Represents the result of an integration test"""
    test_name: str
    passed: bool
    message: str
    execution_time: float
    details: Optional[Dict[str, Any]] = None

class EmergenceIntegrationTestSuite:
    """Integration test suite for the complete EMERGENCE system"""
    
    def __init__(self, base_dir: str = ".."):
        self.base_dir = Path(base_dir)
        self.schemas: Dict[str, Any] = {}
        self.validation_schema: Optional[Dict[str, Any]] = None
        self.test_results: List[IntegrationTestResult] = []
        self.load_all_components()
    
    def load_all_components(self) -> None:
        """Load all EMERGENCE components"""
        logger.info("Loading EMERGENCE components for integration testing...")
        
        # Load schemas
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
        
        # Load validation schema
        validation_file = self.base_dir / "validation" / "testing-system-validation.yaml"
        if validation_file.exists():
            try:
                with open(validation_file, 'r') as f:
                    self.validation_schema = yaml.safe_load(f)
                    logger.info("Loaded validation schema")
            except Exception as e:
                logger.error(f"Failed to load validation schema: {e}")
    
    def run_integration_tests(self) -> List[IntegrationTestResult]:
        """Run all integration tests"""
        logger.info("Starting EMERGENCE integration tests...")
        
        test_methods = [method for method in dir(self) if method.startswith('test_')]
        
        for test_method in test_methods:
            test_func = getattr(self, test_method)
            if callable(test_func):
                self._run_integration_test(test_method, test_func)
        
        return self.test_results
    
    def _run_integration_test(self, test_name: str, test_func) -> None:
        """Execute a single integration test"""
        start_time = datetime.now()
        
        try:
            result = test_func()
            execution_time = (datetime.now() - start_time).total_seconds()
            
            test_result = IntegrationTestResult(
                test_name=test_name,
                passed=result.get('passed', False),
                message=result.get('message', ''),
                execution_time=execution_time,
                details=result.get('details', {})
            )
            
            self.test_results.append(test_result)
            
            status = "PASSED" if test_result.passed else "FAILED"
            logger.info(f"{status}: {test_name} - {test_result.message}")
            
        except Exception as e:
            execution_time = (datetime.now() - start_time).total_seconds()
            test_result = IntegrationTestResult(
                test_name=test_name,
                passed=False,
                message=f"Integration test failed with exception: {str(e)}",
                execution_time=execution_time,
                details={'exception': str(e)}
            )
            self.test_results.append(test_result)
            logger.error(f"FAILED: {test_name} - Exception: {e}")
    
    def test_complete_system_architecture(self) -> Dict[str, Any]:
        """Test that all system components work together"""
        required_components = [
            'essences_tester-essence',
            'capabilities_test-generation',
            'capabilities_test-optimization',
            'behaviors_autonomous-testing',
            'physics_testing-environment'
        ]
        
        missing_components = [comp for comp in required_components if comp not in self.schemas]
        
        if missing_components:
            return {
                'passed': False,
                'message': f"Missing system components: {missing_components}",
                'details': {'missing_components': missing_components}
            }
        
        if not self.validation_schema:
            return {
                'passed': False,
                'message': "Validation schema missing",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': "Complete system architecture is valid",
            'details': {
                'components_count': len(self.schemas),
                'validation_configured': bool(self.validation_schema)
            }
        }
    
    def test_essence_capability_integration(self) -> Dict[str, Any]:
        """Test integration between essence and capabilities"""
        essence_schema = self.schemas.get('essences_tester-essence')
        generation_schema = self.schemas.get('capabilities_test-generation')
        optimization_schema = self.schemas.get('capabilities_test-optimization')
        
        if not all([essence_schema, generation_schema, optimization_schema]):
            return {
                'passed': False,
                'message': "Missing required schemas for essence-capability integration",
                'details': {}
            }
        
        # Check that essence capabilities match capability schemas
        essence_capabilities = essence_schema.get('capabilities', {})
        learned_capabilities = essence_capabilities.get('learned', {})
        
        # Check for test generation capability
        if 'code_analysis' not in learned_capabilities:
            return {
                'passed': False,
                'message': "Essence missing code_analysis capability",
                'details': {}
            }
        
        # Check for test optimization capability
        if 'coverage_analysis' not in learned_capabilities:
            return {
                'passed': False,
                'message': "Essence missing coverage_analysis capability",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': "Essence and capabilities are properly integrated",
            'details': {
                'learned_capabilities': len(learned_capabilities),
                'capability_schemas': 2
            }
        }
    
    def test_behavior_physics_integration(self) -> Dict[str, Any]:
        """Test integration between behaviors and physics"""
        behavior_schema = self.schemas.get('behaviors_autonomous-testing')
        physics_schema = self.schemas.get('physics_testing-environment')
        
        if not all([behavior_schema, physics_schema]):
            return {
                'passed': False,
                'message': "Missing required schemas for behavior-physics integration",
                'details': {}
            }
        
        # Check that behavior sequence aligns with physics constraints
        behavior_sequence = behavior_schema.get('behavior_sequence', {})
        physics_constraints = physics_schema.get('constraints', {})
        
        if not behavior_sequence:
            return {
                'passed': False,
                'message': "Behavior sequence is empty",
                'details': {}
            }
        
        if not physics_constraints:
            return {
                'passed': False,
                'message': "Physics constraints are empty",
                'details': {}
            }
        
        # Check that behavior phases respect physics constraints
        required_phases = ['phase_1_analysis', 'phase_2_test_generation', 'phase_3_test_execution']
        missing_phases = [phase for phase in required_phases if phase not in behavior_sequence]
        
        if missing_phases:
            return {
                'passed': False,
                'message': f"Behavior missing required phases: {missing_phases}",
                'details': {'missing_phases': missing_phases}
            }
        
        return {
            'passed': True,
            'message': "Behaviors and physics are properly integrated",
            'details': {
                'behavior_phases': len(behavior_sequence),
                'physics_constraints': len(physics_constraints)
            }
        }
    
    def test_energy_dynamics_integration(self) -> Dict[str, Any]:
        """Test integration of energy dynamics across components"""
        essence_schema = self.schemas.get('essences_tester-essence')
        physics_schema = self.schemas.get('physics_testing-environment')
        
        if not all([essence_schema, physics_schema]):
            return {
                'passed': False,
                'message': "Missing schemas for energy dynamics integration",
                'details': {}
            }
        
        # Check essence energy profile
        essence_energy = essence_schema.get('energy_profile', {})
        if not essence_energy:
            return {
                'passed': False,
                'message': "Essence missing energy profile",
                'details': {}
            }
        
        # Check physics energy dynamics
        physics_energy = physics_schema.get('energy_dynamics', {})
        if not physics_energy:
            return {
                'passed': False,
                'message': "Physics missing energy dynamics",
                'details': {}
            }
        
        # Check that energy sources and drains are defined
        essence_sources = essence_energy.get('energy_sources', [])
        essence_drains = essence_energy.get('energy_drains', [])
        physics_sources = physics_energy.get('energy_sources', [])
        physics_drains = physics_energy.get('energy_drains', [])
        
        if not essence_sources or not essence_drains:
            return {
                'passed': False,
                'message': "Essence energy profile incomplete",
                'details': {}
            }
        
        if not physics_sources or not physics_drains:
            return {
                'passed': False,
                'message': "Physics energy dynamics incomplete",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': "Energy dynamics are properly integrated",
            'details': {
                'essence_sources': len(essence_sources),
                'essence_drains': len(essence_drains),
                'physics_sources': len(physics_sources),
                'physics_drains': len(physics_drains)
            }
        }
    
    def test_learning_integration(self) -> Dict[str, Any]:
        """Test integration of learning mechanisms across components"""
        essence_schema = self.schemas.get('essences_tester-essence')
        generation_schema = self.schemas.get('capabilities_test-generation')
        optimization_schema = self.schemas.get('capabilities_test-optimization')
        
        if not all([essence_schema, generation_schema, optimization_schema]):
            return {
                'passed': False,
                'message': "Missing schemas for learning integration",
                'details': {}
            }
        
        # Check essence learning mechanics
        essence_learning = essence_schema.get('learning_mechanics', {})
        if not essence_learning:
            return {
                'passed': False,
                'message': "Essence missing learning mechanics",
                'details': {}
            }
        
        # Check capability learning mechanisms
        generation_learning = generation_schema.get('learning_mechanisms', {})
        optimization_learning = optimization_schema.get('learning_mechanisms', {})
        
        if not generation_learning:
            return {
                'passed': False,
                'message': "Test generation missing learning mechanisms",
                'details': {}
            }
        
        if not optimization_learning:
            return {
                'passed': False,
                'message': "Test optimization missing learning mechanisms",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': "Learning mechanisms are properly integrated",
            'details': {
                'essence_learning': bool(essence_learning),
                'generation_learning': len(generation_learning),
                'optimization_learning': len(optimization_learning)
            }
        }
    
    def test_emergence_integration(self) -> Dict[str, Any]:
        """Test integration of emergence potentials across components"""
        emergence_found = []
        
        for schema_name, schema in self.schemas.items():
            if 'emergence_potentials' in schema:
                potentials = schema['emergence_potentials']
                if isinstance(potentials, dict):
                    for potential_name, potential_data in potentials.items():
                        if isinstance(potential_data, dict) and 'description' in potential_data:
                            emergence_found.append(f"{schema_name}.{potential_name}")
        
        if not emergence_found:
            return {
                'passed': False,
                'message': "No emergence potentials found across components",
                'details': {}
            }
        
        # Check that emergence potentials are consistent
        emergence_types = set()
        for emergence in emergence_found:
            if 'adaptive' in emergence.lower():
                emergence_types.add('adaptive')
            elif 'predictive' in emergence.lower():
                emergence_types.add('predictive')
            elif 'collaborative' in emergence.lower():
                emergence_types.add('collaborative')
            elif 'self' in emergence.lower():
                emergence_types.add('self_improving')
        
        return {
            'passed': True,
            'message': f"Found {len(emergence_found)} emergence potentials across {len(emergence_types)} types",
            'details': {
                'emergence_potentials': emergence_found,
                'emergence_types': list(emergence_types)
            }
        }
    
    def test_validation_integration(self) -> Dict[str, Any]:
        """Test integration of validation with the complete system"""
        if not self.validation_schema:
            return {
                'passed': False,
                'message': "Validation schema not available",
                'details': {}
            }
        
        # Check that validation dimensions align with system components
        validation_dimensions = self.validation_schema.get('validation_dimensions', {})
        required_dimensions = ['functionality', 'performance', 'quality', 'autonomy']
        
        missing_dimensions = [dim for dim in required_dimensions if dim not in validation_dimensions]
        
        if missing_dimensions:
            return {
                'passed': False,
                'message': f"Validation missing dimensions: {missing_dimensions}",
                'details': {'missing_dimensions': missing_dimensions}
            }
        
        # Check that validation tests align with system capabilities
        validation_tests = self.validation_schema.get('validation_tests', {})
        required_tests = [
            'test_generation_validation',
            'test_optimization_validation',
            'autonomous_behavior_validation'
        ]
        
        missing_tests = [test for test in required_tests if test not in validation_tests]
        
        if missing_tests:
            return {
                'passed': False,
                'message': f"Validation missing tests: {missing_tests}",
                'details': {'missing_tests': missing_tests}
            }
        
        return {
            'passed': True,
            'message': "Validation is properly integrated with the system",
            'details': {
                'validation_dimensions': len(validation_dimensions),
                'validation_tests': len(validation_tests)
            }
        }
    
    def test_constraints_integration(self) -> Dict[str, Any]:
        """Test integration of constraints across components"""
        constraints_found = []
        
        for schema_name, schema in self.schemas.items():
            if 'constraints' in schema:
                constraints = schema['constraints']
                if isinstance(constraints, dict):
                    constraints_found.append(schema_name)
        
        if not constraints_found:
            return {
                'passed': False,
                'message': "No constraints found across components",
                'details': {}
            }
        
        # Check for ethical boundaries
        ethical_boundaries_found = []
        for schema_name in constraints_found:
            schema = self.schemas[schema_name]
            constraints = schema.get('constraints', {})
            if 'ethical_boundaries' in constraints:
                ethical_boundaries_found.append(schema_name)
        
        if not ethical_boundaries_found:
            return {
                'passed': False,
                'message': "No ethical boundaries found across components",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': f"Constraints are properly integrated across {len(constraints_found)} components",
            'details': {
                'components_with_constraints': constraints_found,
                'components_with_ethics': ethical_boundaries_found
            }
        }
    
    def generate_integration_report(self) -> Dict[str, Any]:
        """Generate a comprehensive integration test report"""
        total_tests = len(self.test_results)
        passed_tests = sum(1 for result in self.test_results if result.passed)
        failed_tests = total_tests - passed_tests
        
        total_execution_time = sum(result.execution_time for result in self.test_results)
        
        report = {
            'summary': {
                'total_tests': total_tests,
                'passed_tests': passed_tests,
                'failed_tests': failed_tests,
                'success_rate': passed_tests / total_tests if total_tests > 0 else 0,
                'total_execution_time': total_execution_time
            },
            'test_results': [
                {
                    'test_name': result.test_name,
                    'passed': result.passed,
                    'message': result.message,
                    'execution_time': result.execution_time,
                    'details': result.details
                }
                for result in self.test_results
            ],
            'timestamp': datetime.now().isoformat()
        }
        
        return report

def main():
    """Main function to run integration tests"""
    print("EMERGENCE Integration Test Suite")
    print("=" * 40)
    
    # Initialize integration test suite
    test_suite = EmergenceIntegrationTestSuite()
    
    # Run all integration tests
    results = test_suite.run_integration_tests()
    
    # Generate report
    report = test_suite.generate_integration_report()
    
    # Print summary
    print(f"\nIntegration Test Summary:")
    print(f"Total Tests: {report['summary']['total_tests']}")
    print(f"Passed: {report['summary']['passed_tests']}")
    print(f"Failed: {report['summary']['failed_tests']}")
    print(f"Success Rate: {report['summary']['success_rate']:.2%}")
    print(f"Total Execution Time: {report['summary']['total_execution_time']:.2f}s")
    
    # Print detailed results
    print(f"\nDetailed Integration Results:")
    for result in report['test_results']:
        status = "✓ PASS" if result['passed'] else "✗ FAIL"
        print(f"{status}: {result['test_name']} - {result['message']}")
    
    # Save report to file
    report_file = ".emergence/integration_test_report.json"
    os.makedirs(os.path.dirname(report_file), exist_ok=True)
    
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2)
    
    print(f"\nIntegration report saved to: {report_file}")
    
    # Exit with appropriate code
    if report['summary']['failed_tests'] > 0:
        print("\n❌ Some integration tests failed!")
        sys.exit(1)
    else:
        print("\n✅ All integration tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main() 