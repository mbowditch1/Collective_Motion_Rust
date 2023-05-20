#!/bin/bash
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=48
#SBATCH --mem-per-cpu=3700
#SBATCH --time=24:00:00

module purge
module restore rust

MY_NUM_THREADS=$SLURM_CPUS_PER_TASK

export OMP_NUM_THREADS=$MY_NUM_THREADS

cargo run --release

