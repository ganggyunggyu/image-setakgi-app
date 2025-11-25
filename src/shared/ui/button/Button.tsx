import React from 'react';
import { cn } from '@/shared/lib/cn';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'ghost';
}

export const Button: React.FC<ButtonProps> = ({ className, children, variant = 'primary', ...props }) => {
  const base = cn(
    'px-4 py-2 rounded-md transition-colors',
    variant === 'primary' && 'bg-blue-600 text-white hover:bg-blue-700',
    variant === 'ghost' && 'bg-transparent text-blue-600 hover:bg-blue-50',
    className
  );

  return (
    <button type="button" className={base} {...props}>
      {children}
    </button>
  );
};
