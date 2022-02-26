using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Input;
using Microsoft.CodeAnalysis.Text;
using ReactiveUI;

namespace FileOrganizer.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        public int count;
        public string path;
        public string error;
        public string fileType;
        public float progress;
        public string output;

        public int Count
        {
            get => count;
            set => this.RaiseAndSetIfChanged(ref count, value);
        }

        public string Path
        {
            get => path;
            set => this.RaiseAndSetIfChanged(ref path, value);
        }

        public string Error
        {
            get => error;
            set => this.RaiseAndSetIfChanged(ref error, value);
        }

        public string FileType
        {
            get => fileType;
            set => this.RaiseAndSetIfChanged(ref fileType, value);
        }

        public float Progress
        {
            get => progress;
            set => this.RaiseAndSetIfChanged(ref progress, value);
        }
        
        public ICommand AddOneCommand { get; private set; }
        public ICommand Organize { get; private set; }

        public MainWindowViewModel()
        {
            AddOneCommand = ReactiveCommand.Create(() =>
            {
                Count++;
            });

            Organize = ReactiveCommand.Create(async () =>
            {
                if (optionsValid())
                {
                    if (CheckPathValidity())
                    {
                        Progress = 10;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 20;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 30;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 40;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 50;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 60;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 70;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 80;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 90;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        Progress = 100;
                        await Task.Delay(TimeSpan.FromSeconds(1));
                        
                    }
                }
                
            });


        }

        public bool CheckPathValidity()
        {
            // Error = "File path invalid!";
            return true;
        }

        public bool optionsValid()
        {
            
            return true;
        }

        public void updateProgress()
        {
            
        }
    }
}