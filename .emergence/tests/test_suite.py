#!/usr/bin/env python3
"""
EMERGENCE Autonomous Testing System - Comprehensive Test Suite

This test suite validates the autonomous testing system schemas and functionality.
It tests the tester essence, capabilities, behaviors, physics, and validation components.
"""

import yaml
import json
import os
import sys
import unittest
from pathlib import Path
from typing import Dict, Any, List, Optional
from dataclasses import dataclass
from datetime import datetime
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class TestResult:
    """Represents the result of a test execution"""
    test_name: str
    passed: bool
    message: str
    execution_time: float
    details: Optional[Dict[str, Any]] = None

class EmergenceTestSuite:
    """Comprehensive test suite for the EMERGENCE autonomous testing system"""
    
    def __init__(self, schemas_dir: str = "../schemas"):
        self.schemas_dir = Path(schemas_dir)
        self.test_results: List[TestResult] = []
        self.schemas: Dict[str, Any] = {}
        self.load_schemas()
    
    def load_schemas(self) -> None:
        """Load all schema files from the schemas directory"""
        logger.info("Loading EMERGENCE schemas...")
        
        schema_files = [
            "essences/tester-essence.yaml",
            "capabilities/test-generation.yaml", 
            "capabilities/test-optimization.yaml",
            "behaviors/autonomous-testing.yaml",
            "physics/testing-environment.yaml"
        ]
        
        for schema_file in schema_files:
            file_path = self.schemas_dir / schema_file
            if file_path.exists():
                try:
                    with open(file_path, 'r') as f:
                        schema_data = yaml.safe_load(f)
                        schema_name = schema_file.replace('/', '_').replace('.yaml', '')
                        self.schemas[schema_name] = schema_data
                        logger.info(f"Loaded schema: {schema_name}")
                except Exception as e:
                    logger.error(f"Failed to load schema {schema_file}: {e}")
            else:
                logger.warning(f"Schema file not found: {file_path}")
    
    def run_all_tests(self) -> List[TestResult]:
        """Run all tests in the suite"""
        logger.info("Starting comprehensive EMERGENCE test suite...")
        
        test_methods = [method for method in dir(self) if method.startswith('test_')]
        
        for test_method in test_methods:
            test_func = getattr(self, test_method)
            if callable(test_func):
                self._run_test(test_method, test_func)
        
        return self.test_results
    
    def _run_test(self, test_name: str, test_func) -> None:
        """Execute a single test and record results"""
        start_time = datetime.now()
        
        try:
            result = test_func()
            execution_time = (datetime.now() - start_time).total_seconds()
            
            test_result = TestResult(
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
            test_result = TestResult(
                test_name=test_name,
                passed=False,
                message=f"Test failed with exception: {str(e)}",
                execution_time=execution_time,
                details={'exception': str(e)}
            )
            self.test_results.append(test_result)
            logger.error(f"FAILED: {test_name} - Exception: {e}")
    
    def test_schema_loading(self) -> Dict[str, Any]:
        """Test that all schemas can be loaded correctly"""
        required_schemas = [
            'essences_tester-essence',
            'capabilities_test-generation',
            'capabilities_test-optimization', 
            'behaviors_autonomous-testing',
            'physics_testing-environment'
        ]
        
        missing_schemas = [schema for schema in required_schemas if schema not in self.schemas]
        
        if missing_schemas:
            return {
                'passed': False,
                'message': f"Missing required schemas: {missing_schemas}",
                'details': {'missing_schemas': missing_schemas}
            }
        
        return {
            'passed': True,
            'message': f"Successfully loaded {len(self.schemas)} schemas",
            'details': {'loaded_schemas': list(self.schemas.keys())}
        }
    
    def test_tester_essence_structure(self) -> Dict[str, Any]:
        """Test the structure and validity of the tester essence"""
        schema = self.schemas.get('essences_tester-essence')
        if not schema:
            return {
                'passed': False,
                'message': "Tester essence schema not found",
                'details': {}
            }
        
        required_fields = ['identity', 'personality', 'core_drives', 'capabilities']
        missing_fields = [field for field in required_fields if field not in schema]
        
        if missing_fields:
            return {
                'passed': False,
                'message': f"Missing required fields in tester essence: {missing_fields}",
                'details': {'missing_fields': missing_fields}
            }
        
        # Validate personality traits
        personality = schema.get('personality', {})
        required_traits = ['thoroughness', 'precision', 'creativity', 'skepticism', 'persistence', 'adaptability']
        missing_traits = [trait for trait in required_traits if trait not in personality]
        
        if missing_traits:
            return {
                'passed': False,
                'message': f"Missing required personality traits: {missing_traits}",
                'details': {'missing_traits': missing_traits}
            }
        
        # Validate trait values are between 0 and 1
        invalid_traits = []
        for trait, value in personality.items():
            if not isinstance(value, (int, float)) or value < 0 or value > 1:
                invalid_traits.append(f"{trait}: {value}")
        
        if invalid_traits:
            return {
                'passed': False,
                'message': f"Invalid personality trait values: {invalid_traits}",
                'details': {'invalid_traits': invalid_traits}
            }
        
        return {
            'passed': True,
            'message': "Tester essence structure is valid",
            'details': {'personality_traits': len(personality)}
        }
    
    def test_capability_structures(self) -> Dict[str, Any]:
        """Test the structure and validity of capability schemas"""
        capability_schemas = {
            'test-generation': self.schemas.get('capabilities_test-generation'),
            'test-optimization': self.schemas.get('capabilities_test-optimization')
        }
        
        results = {}
        all_passed = True
        
        for capability_name, schema in capability_schemas.items():
            if not schema:
                results[capability_name] = {
                    'passed': False,
                    'message': f"{capability_name} schema not found"
                }
                all_passed = False
                continue
            
            required_fields = ['capability_id', 'name', 'description', 'core_functions']
            missing_fields = [field for field in required_fields if field not in schema]
            
            if missing_fields:
                results[capability_name] = {
                    'passed': False,
                    'message': f"Missing required fields: {missing_fields}"
                }
                all_passed = False
            else:
                results[capability_name] = {
                    'passed': True,
                    'message': f"{capability_name} structure is valid"
                }
        
        return {
            'passed': all_passed,
            'message': f"Capability structure validation: {sum(1 for r in results.values() if r['passed'])}/{len(results)} passed",
            'details': {'capability_results': results}
        }
    
    def test_behavior_structure(self) -> Dict[str, Any]:
        """Test the structure and validity of behavior schemas"""
        schema = self.schemas.get('behaviors_autonomous-testing')
        if not schema:
            return {
                'passed': False,
                'message': "Autonomous testing behavior schema not found",
                'details': {}
            }
        
        required_fields = ['behavior_id', 'name', 'description', 'behavior_sequence']
        missing_fields = [field for field in required_fields if field not in schema]
        
        if missing_fields:
            return {
                'passed': False,
                'message': f"Missing required fields in behavior: {missing_fields}",
                'details': {'missing_fields': missing_fields}
            }
        
        # Validate behavior sequence
        behavior_sequence = schema.get('behavior_sequence', {})
        if not behavior_sequence:
            return {
                'passed': False,
                'message': "Behavior sequence is empty or missing",
                'details': {}
            }
        
        # Check for required phases
        required_phases = ['phase_1_analysis', 'phase_2_test_generation', 'phase_3_test_execution']
        missing_phases = [phase for phase in required_phases if phase not in behavior_sequence]
        
        if missing_phases:
            return {
                'passed': False,
                'message': f"Missing required behavior phases: {missing_phases}",
                'details': {'missing_phases': missing_phases}
            }
        
        return {
            'passed': True,
            'message': "Behavior structure is valid",
            'details': {'phases_count': len(behavior_sequence)}
        }
    
    def test_physics_structure(self) -> Dict[str, Any]:
        """Test the structure and validity of physics schemas"""
        schema = self.schemas.get('physics_testing-environment')
        if not schema:
            return {
                'passed': False,
                'message': "Testing environment physics schema not found",
                'details': {}
            }
        
        required_fields = ['physics_id', 'name', 'description', 'environment_properties', 'forces']
        missing_fields = [field for field in required_fields if field not in schema]
        
        if missing_fields:
            return {
                'passed': False,
                'message': f"Missing required fields in physics: {missing_fields}",
                'details': {'missing_fields': missing_fields}
            }
        
        # Validate forces
        forces = schema.get('forces', {})
        required_forces = ['coverage_force', 'quality_force', 'efficiency_force', 'robustness_force']
        missing_forces = [force for force in required_forces if force not in forces]
        
        if missing_forces:
            return {
                'passed': False,
                'message': f"Missing required forces: {missing_forces}",
                'details': {'missing_forces': missing_forces}
            }
        
        # Validate force properties
        invalid_forces = []
        for force_name, force_data in forces.items():
            if not isinstance(force_data, dict):
                invalid_forces.append(f"{force_name}: not a dictionary")
                continue
            
            required_force_fields = ['description', 'strength', 'direction']
            missing_force_fields = [field for field in required_force_fields if field not in force_data]
            
            if missing_force_fields:
                invalid_forces.append(f"{force_name}: missing {missing_force_fields}")
        
        if invalid_forces:
            return {
                'passed': False,
                'message': f"Invalid force configurations: {invalid_forces}",
                'details': {'invalid_forces': invalid_forces}
            }
        
        return {
            'passed': True,
            'message': "Physics structure is valid",
            'details': {'forces_count': len(forces)}
        }
    
    def test_energy_dynamics(self) -> Dict[str, Any]:
        """Test energy dynamics across all schemas"""
        # Check tester essence energy profile
        tester_schema = self.schemas.get('essences_tester-essence', {})
        energy_profile = tester_schema.get('energy_profile', {})
        
        if not energy_profile:
            return {
                'passed': False,
                'message': "Energy profile missing in tester essence",
                'details': {}
            }
        
        # Validate energy sources and drains
        energy_sources = energy_profile.get('energy_sources', [])
        energy_drains = energy_profile.get('energy_drains', [])
        
        if not energy_sources:
            return {
                'passed': False,
                'message': "No energy sources defined",
                'details': {}
            }
        
        if not energy_drains:
            return {
                'passed': False,
                'message': "No energy drains defined",
                'details': {}
            }
        
        # Check physics energy dynamics
        physics_schema = self.schemas.get('physics_testing-environment', {})
        physics_energy = physics_schema.get('energy_dynamics', {})
        
        if not physics_energy:
            return {
                'passed': False,
                'message': "Energy dynamics missing in physics",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': "Energy dynamics are properly configured",
            'details': {
                'energy_sources_count': len(energy_sources),
                'energy_drains_count': len(energy_drains),
                'physics_energy_configured': bool(physics_energy)
            }
        }
    
    def test_emergence_potentials(self) -> Dict[str, Any]:
        """Test emergence potentials across schemas"""
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
                'message': "No emergence potentials found in schemas",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': f"Found {len(emergence_found)} emergence potentials",
            'details': {'emergence_potentials': emergence_found}
        }
    
    def test_learning_mechanisms(self) -> Dict[str, Any]:
        """Test learning mechanisms across schemas"""
        learning_found = []
        
        for schema_name, schema in self.schemas.items():
            if 'learning_mechanics' in schema:
                learning_mechanics = schema['learning_mechanics']
                if isinstance(learning_mechanics, dict):
                    learning_found.append(schema_name)
        
        if not learning_found:
            return {
                'passed': False,
                'message': "No learning mechanisms found in schemas",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': f"Found learning mechanisms in {len(learning_found)} schemas",
            'details': {'schemas_with_learning': learning_found}
        }
    
    def test_constraints_and_ethics(self) -> Dict[str, Any]:
        """Test that constraints and ethical boundaries are defined"""
        constraints_found = []
        
        for schema_name, schema in self.schemas.items():
            if 'constraints' in schema:
                constraints = schema['constraints']
                if isinstance(constraints, dict):
                    constraints_found.append(schema_name)
        
        if not constraints_found:
            return {
                'passed': False,
                'message': "No constraints found in schemas",
                'details': {}
            }
        
        return {
            'passed': True,
            'message': f"Found constraints in {len(constraints_found)} schemas",
            'details': {'schemas_with_constraints': constraints_found}
        }
    
    def test_schema_consistency(self) -> Dict[str, Any]:
        """Test consistency across schemas"""
        inconsistencies = []
        
        # Check for consistent naming patterns
        for schema_name, schema in self.schemas.items():
            if 'identity' in schema:
                identity = schema['identity']
                if 'essence_id' in identity:
                    essence_id = identity['essence_id']
                    if not essence_id or not isinstance(essence_id, str):
                        inconsistencies.append(f"{schema_name}: invalid essence_id")
        
        # Check for consistent capability references
        capability_schemas = [name for name in self.schemas.keys() if 'capabilities_' in name]
        if len(capability_schemas) < 2:
            inconsistencies.append("Expected at least 2 capability schemas")
        
        if inconsistencies:
            return {
                'passed': False,
                'message': f"Found {len(inconsistencies)} inconsistencies",
                'details': {'inconsistencies': inconsistencies}
            }
        
        return {
            'passed': True,
            'message': "Schemas are consistent",
            'details': {'capability_schemas': len(capability_schemas)}
        }
    
    def generate_test_report(self) -> Dict[str, Any]:
        """Generate a comprehensive test report"""
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
    """Main function to run the test suite"""
    print("EMERGENCE Autonomous Testing System - Comprehensive Test Suite")
    print("=" * 60)
    
    # Initialize test suite
    test_suite = EmergenceTestSuite()
    
    # Run all tests
    results = test_suite.run_all_tests()
    
    # Generate report
    report = test_suite.generate_test_report()
    
    # Print summary
    print(f"\nTest Summary:")
    print(f"Total Tests: {report['summary']['total_tests']}")
    print(f"Passed: {report['summary']['passed_tests']}")
    print(f"Failed: {report['summary']['failed_tests']}")
    print(f"Success Rate: {report['summary']['success_rate']:.2%}")
    print(f"Total Execution Time: {report['summary']['total_execution_time']:.2f}s")
    
    # Print detailed results
    print(f"\nDetailed Results:")
    for result in report['test_results']:
        status = "✓ PASS" if result['passed'] else "✗ FAIL"
        print(f"{status}: {result['test_name']} - {result['message']}")
    
    # Save report to file
    report_file = ".emergence/test_report.json"
    os.makedirs(os.path.dirname(report_file), exist_ok=True)
    
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2)
    
    print(f"\nDetailed report saved to: {report_file}")
    
    # Exit with appropriate code
    if report['summary']['failed_tests'] > 0:
        print("\n❌ Some tests failed!")
        sys.exit(1)
    else:
        print("\n✅ All tests passed!")
        sys.exit(0)

if __name__ == "__main__":
    main() 