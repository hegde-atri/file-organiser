using System;
using System.Collections.Generic;
using System.Text;
using System.Windows.Input;
using ReactiveUI;

namespace FileOrganizer.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        public int count;

        public int Count
        {
            get => count;
            set => this.RaiseAndSetIfChanged(ref count, value);
        }
        
        public ICommand AddOneCommand { get; private set; }

        public MainWindowViewModel()
        {
            AddOneCommand = ReactiveCommand.Create(() => { Count++;});
        }
    }
}