#!/usr/bin/env python3
"""
EMERGENCE Testing System Validation Tests

This module contains tests specifically for validating the validation schema
and ensuring the testing system validation works correctly.
"""

import yaml
import json
import os
from pathlib import Path
from typing import Dict, Any, List
from dataclasses import dataclass
from datetime import datetime

@dataclass
class ValidationTestResult:
    """Represents the result of a validation test"""
    test_name: str
    passed: bool
    message: str
    details: Dict[str, Any]

class ValidationTestSuite:
    """Test suite for validation schema and functionality"""
    
    def __init__(self, validation_dir: str = ".emergence/validation"):
        self.validation_dir = Path(validation_dir)
        self.validation_schema = None
        self.load_validation_schema()
    
    def load_validation_schema(self) -> None:
        """Load the validation schema"""
        validation_file = self.validation_dir / "testing-system-validation.yaml"
        if validation_file.exists():
            try:
                with open(validation_file, 'r') as f:
                    self.validation_schema = yaml.safe_load(f)
            except Exception as e:
                print(f"Failed to load validation schema: {e}")
    
    def run_validation_tests(self) -> List[ValidationTestResult]:
        """Run all validation tests"""
        results = []
        
        test_methods = [method for method in dir(self) if method.startswith('test_')]
        for test_method in test_methods:
            test_func = getattr(self, test_method)
            if callable(test_func):
                result = test_func()
                results.append(result)
        
        return results
    
    def test_validation_schema_loading(self) -> ValidationTestResult:
        """Test that the validation schema can be loaded"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="validation_schema_loading",
                passed=False,
                message="Validation schema could not be loaded",
                details={'error': 'Schema file not found or invalid'}
            )
        
        return ValidationTestResult(
            test_name="validation_schema_loading",
            passed=True,
            message="Validation schema loaded successfully",
            details={'schema_keys': list(self.validation_schema.keys())}
        )
    
    def test_validation_dimensions(self) -> ValidationTestResult:
        """Test that validation dimensions are properly defined"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="validation_dimensions",
                passed=False,
                message="No validation schema available",
                details={}
            )
        
        dimensions = self.validation_schema.get('validation_dimensions', {})
        required_dimensions = ['functionality', 'performance', 'quality', 'autonomy']
        
        missing_dimensions = [dim for dim in required_dimensions if dim not in dimensions]
        
        if missing_dimensions:
            return ValidationTestResult(
                test_name="validation_dimensions",
                passed=False,
                message=f"Missing validation dimensions: {missing_dimensions}",
                details={'missing_dimensions': missing_dimensions}
            )
        
        # Check that each dimension has metrics and thresholds
        invalid_dimensions = []
        for dim_name, dim_data in dimensions.items():
            if not isinstance(dim_data, dict):
                invalid_dimensions.append(f"{dim_name}: not a dictionary")
                continue
            
            if 'metrics' not in dim_data:
                invalid_dimensions.append(f"{dim_name}: missing metrics")
            
            if 'thresholds' not in dim_data:
                invalid_dimensions.append(f"{dim_name}: missing thresholds")
        
        if invalid_dimensions:
            return ValidationTestResult(
                test_name="validation_dimensions",
                passed=False,
                message=f"Invalid dimension configurations: {invalid_dimensions}",
                details={'invalid_dimensions': invalid_dimensions}
            )
        
        return ValidationTestResult(
            test_name="validation_dimensions",
            passed=True,
            message=f"All {len(dimensions)} validation dimensions are properly configured",
            details={'dimensions': list(dimensions.keys())}
        )
    
    def test_validation_tests(self) -> ValidationTestResult:
        """Test that validation tests are properly defined"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="validation_tests",
                passed=False,
                message="No validation schema available",
                details={}
            )
        
        validation_tests = self.validation_schema.get('validation_tests', {})
        required_tests = [
            'test_generation_validation',
            'test_optimization_validation', 
            'autonomous_behavior_validation',
            'robustness_validation'
        ]
        
        missing_tests = [test for test in required_tests if test not in validation_tests]
        
        if missing_tests:
            return ValidationTestResult(
                test_name="validation_tests",
                passed=False,
                message=f"Missing validation tests: {missing_tests}",
                details={'missing_tests': missing_tests}
            )
        
        # Check that each test has proper structure
        invalid_tests = []
        for test_name, test_data in validation_tests.items():
            if not isinstance(test_data, dict):
                invalid_tests.append(f"{test_name}: not a dictionary")
                continue
            
            if 'description' not in test_data:
                invalid_tests.append(f"{test_name}: missing description")
            
            if 'test_cases' not in test_data:
                invalid_tests.append(f"{test_name}: missing test_cases")
            
            if 'success_criteria' not in test_data:
                invalid_tests.append(f"{test_name}: missing success_criteria")
        
        if invalid_tests:
            return ValidationTestResult(
                test_name="validation_tests",
                passed=False,
                message=f"Invalid test configurations: {invalid_tests}",
                details={'invalid_tests': invalid_tests}
            )
        
        return ValidationTestResult(
            test_name="validation_tests",
            passed=True,
            message=f"All {len(validation_tests)} validation tests are properly configured",
            details={'validation_tests': list(validation_tests.keys())}
        )
    
    def test_success_criteria(self) -> ValidationTestResult:
        """Test that success criteria are properly defined"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="success_criteria",
                passed=False,
                message="No validation schema available",
                details={}
            )
        
        success_criteria = self.validation_schema.get('success_criteria', {})
        required_criteria = [
            'overall_system_health',
            'continuous_improvement',
            'robustness_validation'
        ]
        
        missing_criteria = [crit for crit in required_criteria if crit not in success_criteria]
        
        if missing_criteria:
            return ValidationTestResult(
                test_name="success_criteria",
                passed=False,
                message=f"Missing success criteria: {missing_criteria}",
                details={'missing_criteria': missing_criteria}
            )
        
        # Check that each criterion has proper structure
        invalid_criteria = []
        for crit_name, crit_data in success_criteria.items():
            if not isinstance(crit_data, dict):
                invalid_criteria.append(f"{crit_name}: not a dictionary")
                continue
            
            if 'description' not in crit_data:
                invalid_criteria.append(f"{crit_name}: missing description")
            
            if 'criteria' not in crit_data:
                invalid_criteria.append(f"{crit_name}: missing criteria")
            
            if 'threshold' not in crit_data:
                invalid_criteria.append(f"{crit_name}: missing threshold")
        
        if invalid_criteria:
            return ValidationTestResult(
                test_name="success_criteria",
                passed=False,
                message=f"Invalid success criteria configurations: {invalid_criteria}",
                details={'invalid_criteria': invalid_criteria}
            )
        
        return ValidationTestResult(
            test_name="success_criteria",
            passed=True,
            message=f"All {len(success_criteria)} success criteria are properly configured",
            details={'success_criteria': list(success_criteria.keys())}
        )
    
    def test_validation_frequency(self) -> ValidationTestResult:
        """Test that validation frequency is properly defined"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="validation_frequency",
                passed=False,
                message="No validation schema available",
                details={}
            )
        
        validation_frequency = self.validation_schema.get('validation_frequency', {})
        required_frequencies = [
            'continuous_monitoring',
            'periodic_validation',
            'milestone_validation'
        ]
        
        missing_frequencies = [freq for freq in required_frequencies if freq not in validation_frequency]
        
        if missing_frequencies:
            return ValidationTestResult(
                test_name="validation_frequency",
                passed=False,
                message=f"Missing validation frequencies: {missing_frequencies}",
                details={'missing_frequencies': missing_frequencies}
            )
        
        return ValidationTestResult(
            test_name="validation_frequency",
            passed=True,
            message=f"All {len(validation_frequency)} validation frequencies are defined",
            details={'validation_frequencies': list(validation_frequency.keys())}
        )
    
    def test_validation_reporting(self) -> ValidationTestResult:
        """Test that validation reporting is properly defined"""
        if not self.validation_schema:
            return ValidationTestResult(
                test_name="validation_reporting",
                passed=False,
                message="No validation schema available",
                details={}
            )
        
        validation_reporting = self.validation_schema.get('validation_reporting', {})
        required_reporting = [
            'metrics_dashboard',
            'detailed_reports',
            'alert_system'
        ]
        
        missing_reporting = [rep for rep in required_reporting if rep not in validation_reporting]
        
        if missing_reporting:
            return ValidationTestResult(
                test_name="validation_reporting",
                passed=False,
                message=f"Missing validation reporting: {missing_reporting}",
                details={'missing_reporting': missing_reporting}
            )
        
        return ValidationTestResult(
            test_name="validation_reporting",
            passed=True,
            message=f"All {len(validation_reporting)} validation reporting components are defined",
            details={'validation_reporting': list(validation_reporting.keys())}
        )

def main():
    """Run validation tests"""
    print("EMERGENCE Validation Test Suite")
    print("=" * 40)
    
    test_suite = ValidationTestSuite()
    results = test_suite.run_validation_tests()
    
    passed = sum(1 for result in results if result.passed)
    total = len(results)
    
    print(f"\nValidation Test Results:")
    print(f"Total Tests: {total}")
    print(f"Passed: {passed}")
    print(f"Failed: {total - passed}")
    print(f"Success Rate: {passed/total:.2%}" if total > 0 else "Success Rate: 0%")
    
    print(f"\nDetailed Results:")
    for result in results:
        status = "✓ PASS" if result.passed else "✗ FAIL"
        print(f"{status}: {result.test_name} - {result.message}")
    
    # Save results
    report = {
        'summary': {
            'total_tests': total,
            'passed_tests': passed,
            'failed_tests': total - passed,
            'success_rate': passed/total if total > 0 else 0
        },
        'results': [
            {
                'test_name': result.test_name,
                'passed': result.passed,
                'message': result.message,
                'details': result.details
            }
            for result in results
        ],
        'timestamp': datetime.now().isoformat()
    }
    
    report_file = ".emergence/validation_test_report.json"
    os.makedirs(os.path.dirname(report_file), exist_ok=True)
    
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2)
    
    print(f"\nValidation report saved to: {report_file}")
    
    return 0 if passed == total else 1

if __name__ == "__main__":
    exit(main()) 