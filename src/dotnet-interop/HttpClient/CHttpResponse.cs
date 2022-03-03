using System;
using System.Runtime.InteropServices;

namespace HttpClientInterop;

[StructLayout(LayoutKind.Sequential)]
public struct CHttpResponse
{
    public IntPtr status { get; set; }
    public IntPtr body { get; set; }
}