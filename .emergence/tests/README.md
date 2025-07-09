# EMERGENCE Test Suite Documentation

This directory contains comprehensive tests for the EMERGENCE autonomous testing system. The test suite validates all schemas, components, and system integration to ensure the autonomous testing system works correctly.

## Test Suite Overview

The EMERGENCE test suite consists of four main test modules:

### 1. **test_suite.py** - Core Schema Tests
- Validates schema loading and structure
- Tests tester essence configuration
- Validates capability schemas
- Tests behavior patterns
- Validates physics configuration
- Tests energy dynamics and emergence potentials

### 2. **test_validation.py** - Validation System Tests
- Tests validation schema structure
- Validates validation dimensions
- Tests validation test configurations
- Validates success criteria
- Tests validation frequency and reporting

### 3. **test_integration.py** - Integration Tests
- Tests complete system architecture
- Validates essence-capability integration
- Tests behavior-physics integration
- Validates energy dynamics integration
- Tests learning mechanism integration
- Validates emergence potential integration
- Tests validation system integration

### 4. **test_performance.py** - Performance Tests
- Tests schema loading performance
- Validates essence processing performance
- Tests capability processing performance
- Validates behavior processing performance
- Tests physics processing performance
- Validates memory efficiency
- Tests concurrent processing capabilities

## Quick Start

### Prerequisites

1. **Python 3.8+** installed
2. **Required packages** installed:
   ```bash
   pip install -r requirements.txt
   ```

### Running All Tests

To run the complete test suite:

```bash
cd .emergence/tests
python run_all_tests.py
```

This will:
- Execute all test suites
- Generate individual reports
- Create a comprehensive report
- Display summary results

### Running Individual Test Suites

You can run individual test suites:

```bash
# Core schema tests
python test_suite.py

# Validation tests
python test_validation.py

# Integration tests
python test_integration.py

# Performance tests
python test_performance.py
```

## Test Categories

### Schema Validation Tests

These tests ensure all EMERGENCE schemas are properly structured:

- **Schema Loading**: Verifies all schema files can be loaded
- **Structure Validation**: Checks required fields and data types
- **Value Validation**: Ensures numeric values are within valid ranges
- **Consistency Checks**: Validates cross-schema consistency

### Integration Tests

These tests verify how components work together:

- **Component Integration**: Tests interactions between essences, capabilities, and behaviors
- **Energy Dynamics**: Validates energy flow across the system
- **Learning Integration**: Tests learning mechanisms across components
- **Emergence Integration**: Validates emergence potential coordination

### Performance Tests

These tests ensure the system meets performance requirements:

- **Execution Speed**: Tests processing time for various operations
- **Memory Usage**: Validates memory efficiency
- **CPU Usage**: Tests CPU utilization
- **Concurrent Processing**: Tests multi-threaded operations

### Validation System Tests

These tests validate the validation system itself:

- **Validation Dimensions**: Tests functionality, performance, quality, and autonomy validation
- **Test Configurations**: Validates test case definitions
- **Success Criteria**: Tests success metric definitions
- **Reporting Systems**: Validates report generation

## Test Reports

The test suite generates several types of reports:

### Individual Test Reports
- `test_report.json` - Core schema test results
- `validation_test_report.json` - Validation system test results
- `integration_test_report.json` - Integration test results
- `performance_test_report.json` - Performance test results

### Comprehensive Report
- `comprehensive_test_report.json` - Combined results from all test suites

### Report Structure

Each report contains:

```json
{
  "summary": {
    "total_tests": 15,
    "passed_tests": 14,
    "failed_tests": 1,
    "success_rate": 0.93,
    "total_execution_time": 2.45
  },
  "test_results": [
    {
      "test_name": "test_schema_loading",
      "passed": true,
      "message": "Successfully loaded 5 schemas",
      "execution_time": 0.12,
      "details": {...}
    }
  ],
  "timestamp": "2025-01-10T12:00:00Z"
}
```

## Test Configuration

### Performance Thresholds

The performance tests use the following thresholds:

- **Schema Loading**: < 1.0 second for 10 schema loads
- **Essence Processing**: < 0.5 seconds for complete processing
- **Capability Processing**: < 1.0 second for all capabilities
- **Behavior Processing**: < 0.8 seconds for behavior analysis
- **Physics Processing**: < 1.2 seconds for physics calculations
- **Memory Usage**: < 100MB increase, < 500MB total
- **Concurrent Processing**: < 2.0 seconds for multi-threaded operations

### Validation Thresholds

The validation tests check for:

- **Schema Completeness**: All required fields present
- **Value Ranges**: Numeric values between 0 and 1
- **Structure Consistency**: Consistent naming and organization
- **Integration Completeness**: All components properly connected

## Debugging Tests

### Common Issues

1. **Missing Dependencies**
   ```bash
   pip install PyYAML psutil
   ```

2. **Schema File Not Found**
   - Ensure all schema files exist in `.emergence/schemas/`
   - Check file permissions

3. **Performance Test Failures**
   - System may be under heavy load
   - Try running tests individually
   - Check available memory and CPU

### Verbose Output

To get detailed test output:

```bash
python -u test_suite.py  # Unbuffered output
```

### Debug Mode

For debugging specific tests:

```python
# Add to test method
import logging
logging.basicConfig(level=logging.DEBUG)
```

## Extending the Test Suite

### Adding New Tests

1. **Create Test Method**:
   ```python
   def test_new_feature(self) -> Dict[str, Any]:
       """Test description"""
       # Test implementation
       return {
           'passed': True,
           'message': 'Test passed',
           'details': {...}
       }
   ```

2. **Add to Test Suite**:
   - Add test method to appropriate test class
   - Follow naming convention: `test_*`

3. **Update Documentation**:
   - Document new test in this README
   - Update test categories if needed

### Custom Test Suites

To create a new test suite:

1. **Create Test File**:
   ```python
   # test_custom.py
   class CustomTestSuite:
       def __init__(self):
           # Initialize test suite
           pass
       
       def run_tests(self):
           # Run custom tests
           pass
   ```

2. **Add to Runner**:
   - Update `run_all_tests.py` to include new suite
   - Add report collection logic

## Continuous Integration

### GitHub Actions Example

```yaml
name: EMERGENCE Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Set up Python
      uses: actions/setup-python@v2
      with:
        python-version: 3.9
    - name: Install dependencies
      run: |
        pip install -r .emergence/tests/requirements.txt
    - name: Run tests
      run: |
        cd .emergence/tests
        python run_all_tests.py
    - name: Upload test results
      uses: actions/upload-artifact@v2
      with:
        name: test-results
        path: .emergence/*.json
```

## Test Metrics

### Success Criteria

The test suite is considered successful when:

- **All Test Suites Pass**: No test suite failures
- **High Success Rate**: > 90% individual test success rate
- **Performance Targets Met**: All performance thresholds satisfied
- **Integration Complete**: All components properly integrated

### Quality Gates

- **Schema Validation**: 100% schema validation tests pass
- **Integration Tests**: 100% integration tests pass
- **Performance Tests**: 90% performance tests pass
- **Validation Tests**: 100% validation system tests pass

## Troubleshooting

### Test Failures

1. **Check Dependencies**: Ensure all required packages are installed
2. **Verify Schemas**: Check that all schema files exist and are valid YAML
3. **System Resources**: Ensure adequate memory and CPU for performance tests
4. **File Permissions**: Check write permissions for report generation

### Performance Issues

1. **System Load**: Run tests when system is under minimal load
2. **Memory**: Close other applications to free memory
3. **CPU**: Ensure CPU is not heavily utilized by other processes
4. **Disk Space**: Ensure adequate disk space for report files

### Integration Issues

1. **Schema Consistency**: Check for naming inconsistencies across schemas
2. **Missing Components**: Ensure all required components are present
3. **Data Types**: Verify data types match expected formats
4. **Dependencies**: Check component dependencies are properly defined

## Support

For issues with the test suite:

1. **Check Logs**: Review test output for error messages
2. **Validate Schemas**: Ensure all schema files are valid
3. **Update Dependencies**: Ensure all packages are up to date
4. **System Requirements**: Verify system meets minimum requirements

## Contributing

When contributing to the test suite:

1. **Follow Patterns**: Use existing test patterns and conventions
2. **Add Documentation**: Document new tests and features
3. **Update Requirements**: Add new dependencies to requirements.txt
4. **Test Thoroughly**: Ensure new tests pass consistently
5. **Update Reports**: Ensure new tests generate proper reports 