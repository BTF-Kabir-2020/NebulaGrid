import { InputHTMLAttributes, forwardRef } from 'react';
import { cn } from '@/lib/utils';

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, className, id, ...props }, ref) => {
    const inputId = id || label?.toLowerCase().replace(/\s+/g, '-');
    return (
      <div className="space-y-1">
        {label && (
          <label
            htmlFor={inputId}
            className="text-sm font-medium text-slate-700 dark:text-slate-300"
          >
            {label}
          </label>
        )}
        <input
          ref={ref}
          id={inputId}
          className={cn(
            'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 placeholder-slate-400 transition-colors focus:border-nebula-500 focus:outline-none focus:ring-1 focus:ring-nebula-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white dark:placeholder-slate-500',
            error && 'border-red-500 focus:border-red-500 focus:ring-red-500',
            className,
          )}
          {...props}
        />
        {error && (
          <p className="text-xs text-red-500">{error}</p>
        )}
      </div>
    );
  },
);
