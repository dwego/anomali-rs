# Anomali.rs

Anomali.rs is an open-source data analysis tool written in Rust for finding unusual numerical patterns in financial and public datasets.

The user will provide a dataset, initially in CSV format, and select the column containing the values to be analyzed. The dataset may also be separated into groups, such as organizations, suppliers, cities, categories, or time periods, allowing each group to be examined independently.

Before running the analysis, Anomali.rs will clean and validate the selected values. Invalid entries, zero values, negative signs, currency symbols, and formatting differences will be handled without changing the original dataset. The program will also check whether the data is suitable for Benford's Law, since not every collection of numbers naturally follows its distribution.

For valid datasets, Anomali.rs will compare the observed frequency of the first digits with the frequency predicted by Benford's Law. Later versions will also analyze second digits and the first two digits. Statistical measurements such as mean absolute deviation and chi-square will be used to measure how far the dataset is from the expected distribution.

The analysis will not be limited to Benford's Law. Anomali.rs will also look for repeated values, excessive use of rounded numbers, unusual concentrations around specific amounts, sudden changes over time, and other patterns that may indicate inconsistent or artificial data.

Each detected pattern will contribute to an anomaly score. Instead of returning only a number, the program will explain how the score was calculated and show which records were responsible for each alert. This will allow users to move from a statistical result back to the original rows of the dataset.

After processing the data, Anomali.rs will generate a report containing the observed and expected digit distributions, the statistical results, the groups with the strongest deviations, the detected patterns, and the records that deserve closer inspection. The report will initially be displayed in the command line and may later be exported as an interactive HTML page.

Anomali.rs will not claim that an anomaly proves fraud or manipulation. Its purpose is to reduce large datasets into a smaller and explainable collection of unusual records that can be reviewed by researchers, journalists, auditors, developers, or anyone interested in understanding numerical data.

The first version will focus on providing a fast and reliable command-line workflow for CSV files. As the project evolves, it may support JSON and Parquet files, public data APIs, larger datasets, custom detection rules, and a local web interface for exploring the results.
